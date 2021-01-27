use crate::grace_period;
use crate::pipeline::Pipeline;
use chrono::Utc;
use crossbeam_channel::{bounded, tick, Receiver, Sender};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

// Trying to add to the channel once it's reached capacity will block.
// We're using this as an incredibly light form of backpressure.
// If the channel backs up enough to timeout, then it's under too much load.
// Since we're doing everything in memory, it'd have to be under crazy stress
// for that to happen.
const MAX_CHANNEL_BUFFER: usize = 10000;

#[derive(Debug, Copy, Clone)]
pub struct Event {
    id: u128,
    timestamp: u32,
    complete: bool,
    // Used as a bitfield to keep track of which services have seen this event.
    // This means that there can only be up to 32 services in a pipeline.
    services: u32,
}

pub struct EventBatch {
    pub service_mask: u32,
    pub events: Vec<u128>,
}

pub fn start(pipeline: Pipeline) -> Sender<EventBatch> {
    let (send, recv) = bounded(MAX_CHANNEL_BUFFER);
    info!(
        "starting processor thread with channel buffer of {:?} events",
        MAX_CHANNEL_BUFFER
    );
    thread::spawn(move || process(pipeline, recv));
    send
}

fn process(pipeline: Pipeline, recv: Receiver<EventBatch>) {
    let mut events: Vec<Event> = vec![];

    let complete = pipeline.completed_services_mask();
    let required = pipeline.required_services_mask();

    // Used to batch incoming events that will be applied to the full events list on tick.
    let mut event_set: HashMap<u128, Event> = HashMap::new();

    let mut grace_period =
        grace_period::Detector::new(Utc::now().timestamp() + pipeline.max_seconds_to_reach_end);

    let ticker = tick(Duration::from_secs(1));
    loop {
        select! {
            recv(recv) -> msg => {
                let batch = msg.expect("error trying to recv in process thread");

                // Setting the timestamp via the only consumer guarantees the vec is sorted in
                // by event timestamp. This makes expiration simple.
                let timestamp = Utc::now().timestamp() as u32;

                for id in &batch.events {
                    event_set.entry(*id)
                        .and_modify(|e| { (*e).services |= batch.service_mask })
                        .or_insert(Event{id: *id, timestamp, services: batch.service_mask, complete: false});
                }
            },
            recv(ticker) -> _ => {
                let now = Utc::now().timestamp();

                // Keep track of added and updated events from event_set for logging purposes
                let mut added = 0;
                let mut updated = 0;
                let mut expired = 0;

                // Keep track of incomplete ticks for grace period short circuiting
                let mut completed_events = 0;

                let mut expire_until_idx: isize = -1;
                for (i, ev) in events.iter_mut().enumerate() {
                    let expire_at = ev.timestamp + pipeline.max_seconds_to_reach_end as u32;

                    // Incorporate latest updates from event_set.
                    // We need to remove from event_set so we can add all remaining (and therefore
                    // new) events to the main list at the end.
                    if let Some(event_update) = event_set.remove(&ev.id) {
                            (*ev).services |= event_update.services;
                            updated += 1;
                    }

                    let event_complete = ev.services == complete || (ev.services & required) == required;
                    if event_complete {
                        report_event_status(ev, &*pipeline.name, event_complete, grace_period.within_grace_period(ev.timestamp));
                        ev.complete = true;
                        completed_events += 1;
                    }

                    if expire_at < now as u32 {
                      expire_until_idx = i as isize;
                    }
                }
                events.retain(|&event| !event.complete);
                if expire_until_idx > -1 {
                    let index = expire_until_idx as usize + 1;
                    let unexpired_events = events.drain(index..).collect();
                    for (_, ev) in events.iter_mut().enumerate() {
                        let event_complete = ev.services == complete || (ev.services & required) == required;
                        report_event_status(ev, &*pipeline.name, event_complete, false);
                        if event_complete {
                            completed_events += 1;
                        } else {
                            expired+=1;
                        }
                    }
                    events = unexpired_events;
                }

                // Add all brand new events from event_set to the main event list
                // TODO: There is an `into_values` method in nightly that will be useful here once
                // stabilized
                for v in event_set.values() {
                    events.push(*v);
                    added += 1;
                }

                // Reset event_set
                event_set = HashMap::new();

                if expired > 0 || updated > 0 || added > 0 || completed_events > 0 {
                  info!("expired {:?} events, added {:?} events, updated {:?} events, completed {:?} events for pipeline {:?}", 
                        expired, added, updated, completed_events, pipeline.name);
                }

                if completed_events  > 0 || expired > 0 {
                    grace_period.register_successful_tick();
                }
            },
        }
    }
}

fn report_event_status(ev: &Event, pipeline: &str, complete: bool, in_grace_period: bool) {
    if in_grace_period {
        return;
    }

    let uuid = Uuid::from_u128(ev.id);

    if complete {
        info!(
            "event id {:?} completed pipeline {:?} : {:#018b}",
            uuid, pipeline, ev.services
        );
    } else {
        info!(
            "event id {:?} did not complete pipeline {:?} : {:#018b}",
            uuid, pipeline, ev.services
        );
    }
}

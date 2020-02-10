use crate::pipeline::Pipeline;
use chrono::Utc;
use crossbeam_channel::{bounded, tick, Receiver, Sender};
use std::thread;
use std::time::Duration;

// Trying to add to the channel once it's reached capacity will block.
// We're using this as an incredibly light form of backpressure.
// If the channel backs up enough to timeout, then it's under too much load.
// Since we're doing everything in memory, it'd have to be under crazy stress
// for that to happen.
const MAX_CHANNEL_BUFFER: usize = 100;

pub struct Event {
    id: u128,
    timestamp: u32,

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

    let ticker = tick(Duration::from_secs(1));

    loop {
        select! {
            recv(recv) -> msg => {
                let batch = msg.expect("error trying to recv in process thread");

                // Setting the timestamp via the only consumer guarantees the vec is sorted in
                // by event timestamp. This makes expiration simple.
                let timestamp = Utc::now().timestamp() as u32;

                let mut added = 0;
                let mut updated = 0;

                for id in batch.events {
                    match events.iter().position(|e| e.id == id) {
                        Some(idx) => {
                            events[idx].services |= batch.service_mask;
                            updated += 1;
                        },
                        None => {
                            events.push(Event{ id, timestamp, services: batch.service_mask });
                            added += 1;
                        }
                    };
                }

                info!("added {:?} and updated {:?} events for pipeline {:?}", added, updated, pipeline.name);
            },
            recv(ticker) -> _ => {
                // Expire events and collect stats
                info!("expiring events for pipeline {:?}", pipeline.name);
            },
        }
    }
}

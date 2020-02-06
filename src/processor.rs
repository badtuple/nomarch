use crossbeam_channel::{Sender, Receiver, bounded};
use std::thread;
use std::time::Duration;

// Trying to add to the channel once it's reached capacity will block.
// We're using this as an incredibly light form of backpressure.
// If the channel backs up enough to timeout, then it's under too much load.
// Since we're doing everything in memory, it'd have to be under crazy stress
// for that to happen.
const MAX_CHANNEL_BUFFER: usize = 100_000;

pub struct Event {
    id: u128,
    timestamp: u32,

    // Used as a bitfield to keep track of which services have seen this event.
    // This means that there can only be up to 32 services in a pipeline.
    services: u32,
}

pub struct EventNotification {
    id: u128,
    service: u32,
}

pub fn start() -> Sender<EventNotification> {
    let (send, recv) = bounded(MAX_CHANNEL_BUFFER);
    info!("starting processor thread with channel buffer of {:?} events", MAX_CHANNEL_BUFFER);
    thread::spawn(move || process(recv));
    send
}

fn process(recv: Receiver<EventNotification>) { }

const REQUIRED_SUCCESS_COUNT: usize = 3;

/// The Grace Period Detector determines if an event that is being expired falls within the grace
/// period or not. This is to eliminate the false negatives for events partially reported due to
/// Nomarch starting while they were already part way through the pipeline.
///
/// The Grace Period is at most the length of the Pipeline's `max_seconds_to_reach_end` config
/// value. But if REQUIRED_SUCCESS_COUNT ticks complete, each with 0 incomplete events and at least
/// 1 event expired during the tick, then we consider the system healthy and short-circuit the
/// Grace Period.
///
/// If the Grace Period successfully short circuits, then calls to register_unsuccessful_tick will
/// not re-enable it.
///
/// This makes sure people see results as quickly as possible, without having to configure a
/// fallible heuristic to control the delay in reporting.
pub struct Detector {
    success_count: usize,
    timeout_timestamp: i64,
}

impl Detector {
    pub fn new(timeout_timestamp: i64) -> Self {
        Self {
            success_count: 0,
            timeout_timestamp,
        }
    }

    pub fn register_successful_tick(&mut self) {
        if self.success_count < REQUIRED_SUCCESS_COUNT {
            self.success_count += 1;
        }
    }

    pub fn register_unsuccessful_tick(&mut self) {
        // Unsuccessful ticks should only reset things if the Grace Perod hasn't been short
        // circuited.
        if self.success_count < REQUIRED_SUCCESS_COUNT {
            self.success_count = 0
        }
    }

    pub fn within_grace_period(&self, event_timestamp: u32) -> bool {
        if self.success_count >= REQUIRED_SUCCESS_COUNT {
            return false;
        }

        event_timestamp as i64 <= self.timeout_timestamp
    }
}

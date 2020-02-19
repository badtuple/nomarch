use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pipeline {
    pub name: String,
    pub max_seconds_to_reach_end: u64,
    pub services: Vec<Service>,
}

impl Pipeline {
    pub fn completed_services_mask(&self) -> u32 {
        let num_of_services = self.services.len();
        let mut mask = 0;
        for i in 0..num_of_services {
            mask |= 1 << i;
        }
        mask
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Service {
    pub name: String,
    pub children: Vec<String>,

    #[serde(default)]
    stats: Stats,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Stats {
    events_seen: f64,
    events_expected: f64,
}

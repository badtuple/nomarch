use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pipeline {
    pub name: String,
    pub max_seconds_to_reach_end: i64,
    pub seconds_from_startup_to_ignore_event_evaluation: i64,
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

    pub fn required_services_mask(&self) -> u32 {
        let mut mask = 0;
        for (pos, e) in self.services.iter().enumerate() {
            if e.required {
                mask |= 1 << pos;
            }
        }
        mask
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Service {
    pub name: String,
    pub children: Vec<String>,
    pub required: bool,
}

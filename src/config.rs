use crate::pipeline::Pipeline;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    version: String,
    pub pipelines: Vec<Pipeline>,
}

pub fn load() -> Config {
    info!("loading config from config.json");
    let path = Path::new("config.json");
    let file = File::open(path).expect("config not found");

    serde_json::from_reader(file).expect("error while reading json")
}

impl Config {
    /// returns a bitmask that can be OR'd with an event's bitset to signify that it has been seen
    /// by the specified service.
    pub fn get_service_mask(&self, pipeline_id: &str, service_id: &str) -> Option<u32> {
        let pipeline = self.pipelines.iter().find(|&p| p.name == pipeline_id)?;
        let service_idx = pipeline
            .services
            .iter()
            .position(|s| s.name == service_id)?;

        // toggles bit corresponding to index of service
        let mask: u32 = 1 << service_idx;
        Some(mask)
    }
}

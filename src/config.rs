use crate::pipeline::Pipeline;
use serde::{Serialize, Deserialize};
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

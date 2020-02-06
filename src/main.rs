#[macro_use]
extern crate log;
#[macro_use]
extern crate crossbeam_channel;

mod processor;

use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;

use serde_json;
use std::fs::File;
use std::path::Path;

use env_logger::{Builder, Target};
use log::LevelFilter;
use processor::start;

#[derive(Deserialize, Debug)]
struct Config {
    version: String,
    pipeline: Pipeline,
}

#[derive(Deserialize, Debug)]
struct Pipeline {
    name: String,
    max_seconds_to_reach_end: u64,
    services: Vec<String>,
}

fn load_config() -> Config {
    info!("loading config from config.json");
    let path = Path::new("config.json");
    let file = File::open(path).expect("config not found");

    serde_json::from_reader(file).expect("error while reading json")
}

fn setup_logger() {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .target(Target::Stdout)
        .init();
}

#[get("/")]
async fn health() -> impl Responder {
    r#"{"status": "pass", "version": "v0.0.1", "service_id": "nomarch"}"#
}

#[derive(Deserialize, Debug)]
struct EventRequest {
    service: String,
    events: Vec<Event>,
}

#[derive(Deserialize, Debug)]
struct Event {
    timestamp: u64,
    id: String,
}

#[post("/events")]
async fn register_events(req: web::Json<EventRequest>) -> impl Responder {
    info!("events: {:?}", req);
    "{}"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    setup_logger();
    info!("starting nomarch");

    let config = load_config();
    info!("configuration loaded: {:?}", config);

    info!("server started on port 8080");
    HttpServer::new(|| App::new().service(health).service(register_events))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

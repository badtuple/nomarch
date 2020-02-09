#[macro_use]
extern crate log;
#[macro_use]
extern crate crossbeam_channel;

mod config;
mod pipeline;
mod processor;

use actix_web::{get, middleware::Logger, post, web, App, HttpServer, Responder};
use env_logger::{Builder, Target};
use log::LevelFilter;
use processor::start;
use serde::Deserialize;
use config::Config;

fn setup_logger() {
    Builder::new()
        .filter(None, LevelFilter::Debug)
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

#[get("/pipelines")]
async fn pipeline_handler(data: web::Data<Config>) -> impl Responder {
    let config = data.get_ref();
    web::Json(config.clone())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    setup_logger();
    info!("starting nomarch");

    let config = config::load();
    info!("configuration loaded: {:?}", config);

    info!("server started on port 8080");
    HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .wrap(Logger::default())
            .service(health)
            .service(pipeline_handler)
            .service(register_events)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

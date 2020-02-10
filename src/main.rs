#[macro_use]
extern crate log;
#[macro_use]
extern crate crossbeam_channel;

mod config;
mod pipeline;
mod processor;

use actix_web::{get, middleware::Logger, post, web, App, HttpServer, Responder};
use config::Config;
use crossbeam_channel::Sender;
use env_logger::{Builder, Target};
use log::LevelFilter;
use processor::EventBatch;
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

fn setup_logger() {
    Builder::new()
        .filter(None, LevelFilter::Debug)
        .target(Target::Stdout)
        .init();
}

#[get("/")]
async fn health_handler() -> impl Responder {
    r#"{"status": "pass", "version": "v0.0.1", "service_id": "nomarch"}"#
}

#[derive(Deserialize, Debug)]
struct EventRequest {
    pipeline: String,
    service: String,
    events: Vec<String>,
}

#[post("/events")]
async fn event_handler(
    req: web::Json<EventRequest>,
    data: web::Data<SharedState>,
) -> impl Responder {
    let config = data.get_ref().config.clone();
    let sender = match data.get_ref().senders.get(&req.pipeline) {
        Some(send) => send,
        None => return "{\"error\": \"pipeline not found\"}",
    };

    let service_mask = match config.get_service_mask(&*req.pipeline, &*req.service) {
        Some(mask) => mask,
        None => return "{\"error\": \"service not found\"}",
    };

    let events = req
        .events
        .iter()
        .filter_map(|raw| match Uuid::parse_str(&*raw) {
            Ok(uuid) => Some(uuid.as_u128()),
            Err(_) => None,
        })
        .collect();

    sender
        .send(EventBatch {
            service_mask,
            events,
        })
        .expect("could not send event batch");

    "{}"
}

#[get("/pipelines")]
async fn pipeline_handler(data: web::Data<SharedState>) -> impl Responder {
    let config = data.get_ref().config.clone();
    web::Json(config)
}

struct SharedState {
    config: Config,
    senders: HashMap<String, Sender<EventBatch>>,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    setup_logger();
    info!("starting nomarch");

    let config = config::load();
    info!("configuration loaded: {:?}", config);

    let mut senders = HashMap::new();
    for pipeline in &config.pipelines {
        let sender = processor::start(pipeline.clone());
        senders.insert(pipeline.name.clone(), sender);
    }

    info!("server started on port 8080");
    HttpServer::new(move || {
        App::new()
            .data(SharedState {
                config: config.clone(),
                senders: senders.clone(),
            })
            .wrap(Logger::default())
            .service(health_handler)
            .service(pipeline_handler)
            .service(event_handler)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

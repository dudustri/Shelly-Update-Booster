mod config;
mod client;
mod store;
mod consumer;

use std::sync::Arc;

use config::load_config;
use store::init_storage;
use client::run_mqtt_client;
use tokio::{self, sync::mpsc};

#[tokio::main]
async fn main() {
    let config = load_config();

    let storage = init_storage(&config.mongo_uri, &config.mongo_db, &config.mongo_collection)
        .await
        .expect("Failed to initialize storage");

    let storage = Arc::new(storage);
    
    let (produce, consume) = mpsc::channel::<String>(100);
    consumer::spawn_consumers(consume, storage, config.consumers_amount).await;

    run_mqtt_client(&config.broker_url, &config.topic, config.broker_port , produce).await;
}

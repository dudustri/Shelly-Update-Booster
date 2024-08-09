mod config;
mod client;
mod store;

use config::load_config;
use store::init_storage;
use client::run_mqtt_client;
use tokio;

#[tokio::main]
async fn main() {
    let config = load_config();

    let storage = init_storage(&config.mongo_uri, &config.mongo_db, &config.mongo_collection)
        .await
        .expect("Failed to initialize storage");

    run_mqtt_client(&config.broker_url, &config.topic, config.broker_port , storage).await;
}

use dotenv::dotenv;
use std::env;

pub struct Config {
    pub broker_url: String,
    pub broker_port: u16,
    pub topic: String,
    pub mongo_uri: String,
    pub mongo_db: String,
    pub mongo_collection: String,
}

pub fn load_config() -> Config {
    dotenv().ok();

    Config {
        broker_url: env::var("MQTT_BROKER_URL").expect("MQTT_BROKER_URL must be set"),
        broker_port: env::var("MQTT_BROKER_PORT").expect("MQTT_BROKER_PORT must be set").parse::<u16>().expect("MQTT_BROKER_PORT must be a valid u16"),
        topic: env::var("MQTT_TOPIC").unwrap_or_else(|_| "shellyUpdateBooster/#".to_string()),
        mongo_uri: env::var("MONGO_URI").expect("MONGO_URI must be set"),
        mongo_db: env::var("MONGO_DB").expect("MONGO_DB must be set"),
        mongo_collection: env::var("MONGO_COLLECTION").expect("MONGO_COLLECTION must be set"),
    }
}
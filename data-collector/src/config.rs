use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub broker_url: String,
    pub broker_port: u16,
    pub topic: String,
    pub mongo_uri: String,
    pub mongo_db: String,
    pub consumers_amount: u8,
}

pub fn load_config() -> Config {
    dotenv().expect(".env file not found");

    Config {
        broker_url: env::var("MQTT_BROKER_URL").expect("MQTT_BROKER_URL must be set"),
        broker_port: env::var("MQTT_BROKER_PORT").expect("MQTT_BROKER_PORT must be set").parse::<u16>().expect("MQTT_BROKER_PORT must be a valid u16"),
        topic: env::var("MQTT_TOPIC").unwrap_or_else(|_| "shellyUpdateBooster/#".to_string()),
        mongo_uri: env::var("MONGO_URI").expect("MONGO_URI must be set"),
        mongo_db: env::var("MONGO_DB").expect("MONGO_DB must be set"),
        consumers_amount: env::var("CONSUMERS_AMOUNT").unwrap_or_else(|_|  "5".to_string()).parse::<u8>().expect("CONSUMER_AMOUNT should be a uint 8"),
    }
}

use rumqttc::{Client, Event, Incoming, MqttOptions, QoS};
use tokio::sync::mpsc;
use std::time::Duration;

use crate::models::message::StoreTaskMessage;

pub async fn run_mqtt_client(
    broker_url: &str,
    topic: &str,
    port: u16,
    queue: mpsc::Sender<StoreTaskMessage>
) {
    let mut mqttoptions = MqttOptions::new("bodil_data_collector", broker_url, port);
    mqttoptions.set_keep_alive(Duration::from_secs(20));

    let (client, mut connection) = Client::new(mqttoptions, 10);

    client.subscribe(topic, QoS::AtMostOnce).unwrap();

    loop {
        match connection.eventloop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(packet))) => {
                if let Ok(payload) = std::str::from_utf8(&packet.payload){
                    // println!("Received message: {} - from topic: {}", payload, &packet.topic);
                    match extract_collection_from_topic_after_wildcard(topic, &packet.topic){
                        Ok(collection) => {
                            println!("Extracted collection: {}", collection);
                            let store_message = StoreTaskMessage{
                                collection: collection,
                                payload: payload.to_string(),
                            };
                            if let Err(e) = queue.send(store_message).await {
                                eprintln!("Failed to send message to the worker: {}", e);
                            }
                        },
                        Err(e) => eprintln!("Failed to extract collection: {}", e),
                    }
                }
            }
            Ok(_) => {}
            Err(e) => eprintln!("Error in event loop: {}", e),
        }
    }
}

fn extract_collection_from_topic_after_wildcard(topic_wildcard: &str, topic_received: &str) -> Result<String, String> {
    if topic_wildcard.ends_with('#') {
        if let Some(prefix) = topic_wildcard.strip_suffix('#') {
            if topic_received.starts_with(prefix) {
                if let Some(collection_name) = topic_received.strip_prefix(prefix) {
                    return Ok(collection_name.to_string());
                }
                else{
                    return Err("Error: failed to remove the topic prefix from the collection name.".to_string());
                }
            } else {
                return Err("Error: the topic received does not start with subscribed one prefix.".to_string());
            }
        } else {
            return Err("Error: invalid wildcard structure - # is missing at the end.".to_string());
        }
    } else {
        return Err("Error: wildcard must be at the end of topic_wildcard.".to_string());
    }
}

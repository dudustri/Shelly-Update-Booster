use rumqttc::{Client, Event, Incoming, MqttOptions, QoS};
use tokio::sync::mpsc;
use std::time::Duration;

pub async fn run_mqtt_client(
    broker_url: &str,
    topic: &str,
    port: u16,
    channel_sender: mpsc::Sender<String>
) {
    let mut mqttoptions = MqttOptions::new("bodil_data_collector", broker_url, port);
    mqttoptions.set_keep_alive(Duration::from_secs(20));

    let (client, mut connection) = Client::new(mqttoptions, 10);

    client.subscribe(topic, QoS::AtMostOnce).unwrap();

    loop {
        match connection.eventloop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(publish))) => {
                if let Ok(payload) = std::str::from_utf8(&publish.payload) {
                    println!("Received message: {}", payload);
                    if let Err(e) = channel_sender.send(payload.to_string()).await {
                        eprintln!("Failed to send message to the worker: {}", e);
                    }
                }
            }
            Ok(_) => {}
            Err(e) => eprintln!("Error in event loop: {}", e),
        }
    }
}

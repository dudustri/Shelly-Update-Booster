use rumqttc::{Client, Event, Incoming, MqttOptions, QoS};
use tokio::sync::mpsc;
use std::time::Duration;
// use models::Message;

pub async fn run_mqtt_client(
    broker_url: &str,
    topic: &str,
    port: u16,
    queue: mpsc::Sender<String>
) {
    let mut mqttoptions = MqttOptions::new("bodil_data_collector", broker_url, port);
    mqttoptions.set_keep_alive(Duration::from_secs(20));

    let (client, mut connection) = Client::new(mqttoptions, 10);

    client.subscribe(topic, QoS::AtMostOnce).unwrap();

    loop {
        match connection.eventloop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(packet))) => {
                if let Ok(payload) = std::str::from_utf8(&packet.payload){
                    println!("Received message: {} - from topic: {}", payload, &packet.topic);
                    //Create the message struct here passing the topic and the payload. Send it to the queue.
                    if let Err(e) = queue.send(payload.to_string()).await {
                        eprintln!("Failed to send message to the worker: {}", e);
                    }
                }
            }
            Ok(_) => {}
            Err(e) => eprintln!("Error in event loop: {}", e),
        }
    }
}

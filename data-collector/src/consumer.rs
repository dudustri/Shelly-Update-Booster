use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use crate::{models::message::StoreTaskMessage, store::storage::Storage};
use tokio::task;

const MAX_RETRIES: u8 = 3;

pub async fn spawn_consumers(
    consumer_queue: mpsc::Receiver<StoreTaskMessage>,
    producer_queue: mpsc::Sender<StoreTaskMessage>,
    storage: Arc<Box<dyn Storage>>,
    worker_count: u8,
) {

    let consumer_queue_with_mutex = Arc::new(Mutex::new(consumer_queue));
    let producer_queue_with_mutex = Arc::new(Mutex::new(producer_queue));

    for thread_id in 0..worker_count {

        let consumer_queue_clone = Arc::clone(&consumer_queue_with_mutex);
        let producer_queue_clone = Arc::clone(&producer_queue_with_mutex);
        let storage_clone = Arc::clone(&storage);

        task::spawn(run_consumer(thread_id, consumer_queue_clone, producer_queue_clone, storage_clone));

        // task::spawn(async move {
        //     println!("Consumer {} created!", thread_id);
        //     while let Some(task_message) = {
        //         let mut queue = queue_clone.lock().await;
        //         queue.recv().await
        //     } {
        //         println!("Thread: {} consumed from the queue and is processing the storage...", thread_id);
        //         if let Err(e) = storage_clone.store(&task_message.collection, &task_message.payload).await {
        //             eprintln!("Failed to store message: {}", e);
        //         }
        //     }
        //     println!("Worker with id {} shutting down because the channel was closed.", thread_id);
        // });

    }
}


async fn process_message(
    storage: Arc<Box<dyn Storage>>,
    task_message: StoreTaskMessage,
) -> Result<(), String> {
    let mut attempts = 0;

    while attempts < MAX_RETRIES {
        match storage.store(&task_message.collection, &task_message.payload).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                eprintln!("Failed to store message: {}. Attempt {} of {}", e, attempts + 1, MAX_RETRIES);
                attempts += 1;
            }
        }
    }

    Err(format!("Failed to store message after {} attempts", MAX_RETRIES))
}

async fn run_consumer(
    thread_id: u8,
    queue_consumer: Arc<Mutex<mpsc::Receiver<StoreTaskMessage>>>,
    queue_producer: Arc<Mutex<mpsc::Sender<StoreTaskMessage>>>,
    storage_clone: Arc<Box<dyn Storage>>,
) {
    println!("Consumer {} created!", thread_id);

    while let Some(task_message) = {
        let mut consume = queue_consumer.lock().await;
        consume.recv().await
    } {
        println!("Thread: {} consumed from the queue and is processing the storage...", thread_id);

        if let Err(e) = process_message(storage_clone.clone(), task_message.clone()).await {
            eprintln!("It wasn't possible to store the data - thread: {} - {}", thread_id, e);
            // push it back to the queue after failure in a retries
            println!("Pushing it back to the queue!");
            let produce_back = queue_producer.lock().await;
            if let Err(e) = produce_back.send(task_message).await {
                eprintln!("Thread: {} - failed to requeue the message: {}", thread_id, e);
            }
        }
    }
    println!("worker with id {} shutting down because the channel was closed.", thread_id);
}
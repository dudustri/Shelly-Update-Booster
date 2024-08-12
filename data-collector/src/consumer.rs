use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};
use crate::store::storage::Storage;
use tokio::task;

pub async fn spawn_consumers(
    consumer_queue: mpsc::Receiver<String>,
    storage: Arc<Box<dyn Storage>>,
    worker_count: u8,
) {

    let consumer_queue_with_mutex = Arc::new(Mutex::new(consumer_queue));

    for thread_id in 0..worker_count {

        let queue_clone = Arc::clone(&consumer_queue_with_mutex);
        let storage_clone = Arc::clone(&storage);

        task::spawn(async move {
            println!("Consumer {} created!", thread_id);
            while let Some(payload) = {
                let mut queue = queue_clone.lock().await;
                queue.recv().await
            } {
                println!("Thread: {} consumed from the queue and is processing the storage...", thread_id);
                if let Err(e) = storage_clone.store(&payload).await {
                    eprintln!("Failed to store message: {}", e);
                }
            }
            println!("Worker with id {} shutting down because the channel was closed.", thread_id);
        });

    }
}

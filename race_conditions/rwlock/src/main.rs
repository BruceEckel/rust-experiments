use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() {
    let counter = Arc::new(tokio::sync::RwLock::new(0)); // RwLock will allow multiple reads but exclusive write.

    async fn update_counter(counter: Arc<tokio::sync::RwLock<i32>>) {
        for _ in 0..1000 {
            let mut value = counter.write().await;  // Acquire a write lock.
            *value += 1;
            // Yield to allow other tasks to run, increasing the chance of a race condition:
            tokio::task::yield_now().await;
        }
    }

    tokio::try_join!(
        tokio::spawn(update_counter(counter.clone())),
        tokio::spawn(update_counter(counter.clone()))
    ).unwrap();

    println!("Result: {}", *counter.read().await);
}

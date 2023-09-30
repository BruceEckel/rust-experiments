use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() {
    let counter = Arc::new(AtomicI32::new(0));

    async fn update_counter(counter: Arc<AtomicI32>) {
        for _ in 0..1000 {
            counter.fetch_add(1, Ordering::Relaxed);

            // Yield to allow other tasks to run, increasing the chance of a race condition:
            tokio::task::yield_now().await;
        }
    }

    tokio::try_join!(
        tokio::spawn(update_counter(counter.clone())),
        tokio::spawn(update_counter(counter.clone()))
    ).unwrap();

    println!("Result: {}", counter.load(Ordering::Relaxed));
}

use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));

    async fn update_counter(counter: Arc<Mutex<i32>>) {
        for _ in 0..1000 {
            let mut num = counter.lock().await;
            *num += 1;
            // Release the lock and yield to allow other tasks to run:
            drop(num);
            tokio::task::yield_now().await;
        }
    }

    tokio::try_join!(
        tokio::spawn(update_counter(counter.clone())),
        tokio::spawn(update_counter(counter.clone()))
    ).unwrap();

    println!("Result: {}", *counter.lock().await);
}

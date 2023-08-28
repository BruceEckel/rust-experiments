use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

async fn fails(id: u32) -> Result<(), &'static str> {
    sleep(Duration::from_secs(1)).await;
    if id % 2 == 0 {
        return Err("Task failed");
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let shared_data = Arc::new(Mutex::new(0));

    let data_clone = shared_data.clone();
    let task_handle = tokio::spawn(async move {
        let lock = data_clone.lock().await; // Acquire lock asynchronously
        let _result = fails(1).await;
        drop(lock); // Release the lock explicitly
    });

    match task_handle.await {
        Ok(_) => println!("Task completed or panicked."),
        Err(_) => println!("Task panicked."),
    }

    println!("Lock should be released at this point.");
}

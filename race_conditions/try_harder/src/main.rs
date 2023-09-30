use std::sync::Arc;
use tokio;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let counter =
        Arc::new(tokio::sync::RwLock::new(0));

    async fn update_counter(
        counter: Arc<tokio::sync::RwLock<i32>>,
    ) {
        for _ in 0..100 {
            // Acquire a write lock:
            let mut value =
                counter.write().await;
            *value += 1;
            // Delay creates more contention:
            tokio::time::sleep(
                Duration::from_micros(10),
            )
            .await;
            // 'value' lock released here
        }
    }

    let mut attempts = 0;
    loop {
        attempts += 1;
        let counter_clone = counter.clone();
        *counter_clone.write().await = 0;

        let num_tasks = 50;
        let mut handles = vec![];
        for _ in 0..num_tasks {
            let counter_clone =
                counter.clone();
            handles.push(tokio::spawn(
                update_counter(counter_clone),
            ));
        }
        for handle in handles {
            handle.await.unwrap();
        }
        let result = *counter.read().await;
        if result != 100 * num_tasks {
            println!(
                "Race @ attempt {}: {} != {}",
                attempts,
                100 * num_tasks,
                result
            );
            break;
        }
    }
}

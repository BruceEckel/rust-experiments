use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() {
    // RwLock allows multiple reads but
    // exclusive write:
    let counter =
        Arc::new(tokio::sync::RwLock::new(0));

    async fn update_counter(
        counter: Arc<tokio::sync::RwLock<i32>>,
    ) {
        for _ in 0..1000 {
            // Acquire a write lock:
            let mut value =
                counter.write().await;
            *value += 1;
            // Yield allows other tasks to
            // run, increasing the chance of a
            // race condition:
            tokio::task::yield_now().await;
            // 'value' lock is released here
        }
    }

    let mut attempts = 0;
    loop {
        attempts += 1;
        let counter_clone = counter.clone();
        *counter_clone.write().await = 0;

        let num_tasks = 10;
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
        if result != 1000 * num_tasks {
            println!(
                "Race @ attempt {}: {} != {}",
                attempts,
                1000 * num_tasks,
                result
            );
            break;
        }
    }
}

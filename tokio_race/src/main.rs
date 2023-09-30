use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));

    let handle1 = {
        let counter = counter.clone();
        tokio::spawn(async move {
            for _ in 0..1000 {
                let mut num =
                    counter.lock().await;
                *num += 1;
                // Release the lock, allowing
                // other tasks to potentially
                // modify the counter:
                drop(num);
                // Yield to allow other tasks
                // to run, increasing the
                // chance of a race condition:
                tokio::task::yield_now().await;
            }
        })
    };

    let handle2 = {
        let counter = counter.clone();
        tokio::spawn(async move {
            for _ in 0..1000 {
                let mut num =
                    counter.lock().await;
                *num += 1;
                drop(num);
                tokio::task::yield_now().await;
            }
        })
    };

    handle1.await.unwrap();
    handle2.await.unwrap();

    println!(
        "Result: {}",
        *counter.lock().await
    );
}

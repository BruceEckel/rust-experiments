use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        for i in 0..10 {
            println!("Task step {}", i);
            sleep(Duration::from_millis(500)).await;
        }
    });

    // Wait for a short duration
    sleep(Duration::from_millis(1500)).await;

    // Drop the handle, which will cancel the task
    drop(handle);

    // Wait for a while to observe the effect (task should be cancelled by now)
    sleep(Duration::from_secs(2)).await;
}

// The async/.await model in Rust is built around cooperative cancellation,
// where tasks are responsible for checking and responding to cancellation
// signals. This model encourages explicit handling of cancellation points
// in your asynchronous code, as it promotes better control and understanding
// of task lifetimes and cancellation points.
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::task;
use tokio::time::sleep;

async fn time_consuming_task(mut cancel_signal: oneshot::Receiver<()>) {
    // Simulate a time-consuming task
    for i in 1..=10 {
        println!("Task running: Step {}", i);
        sleep(Duration::from_secs(1)).await;

        // Check for cancellation signal
        if let Ok(_) = cancel_signal.try_recv() {
            println!("Task cancelled.");
            return;
        }
    }

    println!("Task completed successfully.");
}

#[tokio::main]
async fn main() {
    // Create a cancellation signal channel
    let (cancel_tx, cancel_rx) = oneshot::channel();

    // Spawn the time-consuming task
    let task_handle = task::spawn(time_consuming_task(cancel_rx));

    // Wait for a few seconds, then cancel the task
    sleep(Duration::from_secs(5)).await;

    // Cancel the task by sending a signal
    // if cancel_tx.send(()).is_err() {
    //     println!("Failed to send cancellation signal.");
    // }
    cancel_tx.send(()).unwrap();

    // Await the task to finish
    // if let Err(e) = task_handle.await {
    //     println!("Task panicked: {:?}", e);
    // }
    task_handle.await.unwrap();
}

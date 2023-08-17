use std::sync::{Arc, Mutex};
use tokio::select;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Create a shared cleanup tracker using an Arc and Mutex
    let cleanup = Arc::new(Mutex::new(CleanupOnCancel {
        // Initialize resources that need cleanup here
    }));

    // Spawn the work task
    let work_task = tokio::spawn(work(Arc::clone(&cleanup)));

    // Use the select! macro to wait for either the work task or the timeout
    select! {
        result = work_task => {
            if let Err(_) = result {
                println!("Work task completed with an error");
            } else {
                println!("Work task completed successfully");
            }
        }
        _ = tokio::time::sleep(Duration::from_secs(2)) => {
            println!("Work task was cancelled due to timeout");
        }
    }

    // Cleanup will happen automatically when `cleanup` goes out of scope
}

struct CleanupOnCancel {
    // Add fields for any resources that need cleanup
}

impl Drop for CleanupOnCancel {
    fn drop(&mut self) {
        // Perform cleanup actions here
        println!("Cleaning up after cancellation...");
    }
}

async fn work(_cleanup: Arc<Mutex<CleanupOnCancel>>) -> Result<(), &'static str> {
    // Simulate some computation
    for i in 1..=5 {
        println!("Working... Step {}", i);
        sleep(Duration::from_secs(1)).await;
    }

    // Simulate a failure in the work task
    Err("Work task failed")
}

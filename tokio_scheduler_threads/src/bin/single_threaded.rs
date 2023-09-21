use tokio::task;
use std::time::Instant;
mod cpu_task;  // Import the module
use cpu_task::cpu_bound_task;  // Using the function

#[tokio::main(flavor = "current_thread")] // Single-threaded scheduler
async fn main() {
    let start = Instant::now();
    let t1 = task::spawn_blocking(|| cpu_bound_task("Task one", 1_000_000_000));
    let t2 = task::spawn_blocking(|| cpu_bound_task("Task two", 1_000_000_000));

    let _ = tokio::join!(t1, t2);
    println!("Duration: {:?}", start.elapsed());
}

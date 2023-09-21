mod cpu_task;
use cpu_task::run_tasks;
use tokio::runtime::Builder;

fn main() {
    // Create a single-threaded Tokio runtime
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    run_tasks(rt);
}

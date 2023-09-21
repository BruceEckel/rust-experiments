mod cpu_task;
use cpu_task::run_tasks;
use tokio::runtime::Builder;

fn main() {
    // Create a multi-threaded Tokio runtime
    let rt = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .build()
        .unwrap();

    run_tasks(rt);
}

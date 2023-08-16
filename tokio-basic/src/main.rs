use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::{sleep, Duration};

async fn scenario1() {
    println!("Scenario 1");
    let tasks = (0..5).map(|i| {
        task::spawn(async move {
            println!("Task {} started", i);
            sleep(Duration::from_secs(1 as u64)).await;
            println!("Task {} completed", i);
        })
    });

    // Starts and runs each task to completion before
    // starting the next one:
    for task in tasks {
        task.await.unwrap();
    }
}

struct TaskIterator {
    tasks: Vec<Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>>,
    index: usize,
}

impl Iterator for TaskIterator {
    type Item = Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tasks.len() {
            let handle = self.tasks[self.index].clone();
            self.index += 1;
            Some(handle)
        } else {
            None
        }
    }
}

fn create_tasks() -> TaskIterator {
    let mut tasks: Vec<Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>> = Vec::new();
    for i in 0..5 {
        tasks.push(Arc::new(Mutex::new(Some(tokio::spawn(async move {
            println!("Task {} started", i);
            sleep(Duration::from_secs(1)).await;
            println!("Task {} completed", i);
        })))));
    }
    TaskIterator { tasks, index: 0 }
}

async fn scenario2() {
    println!("Scenario 2");
    // This starts all tasks at once:
    // let tasks = create_tasks();
    for task in create_tasks() {
        let handle = task.lock().await.take().unwrap();
        handle.await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    scenario1().await;
    println!();
    scenario2().await;
}

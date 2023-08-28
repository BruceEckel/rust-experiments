use tokio::task;

async fn sometimes_fails(id: u32) -> Result<(), &'static str> {
    // Simulate a task that may fail
    if id % 2 == 0 {
        return Err("Task failed");
    }
    Ok(()) // Completed successfully
}

#[tokio::main]
async fn main() {
    let tasks = vec![
        task::spawn(sometimes_fails(1)),
        task::spawn(sometimes_fails(2)),
        task::spawn(sometimes_fails(3)),
        task::spawn(sometimes_fails(4)),
        task::spawn(sometimes_fails(5)),
        task::spawn(sometimes_fails(6)),
    ];

    let results: Vec<_> = futures::future::join_all(tasks).await;

    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(res) => match res {
                Ok(_) => println!("Task {} Success", i + 1),
                Err(err_msg) => println!("Task {}: {}", i + 1, err_msg),
            },
            Err(join_err) => println!("Task {} encountered a join error: {:?}", i + 1, join_err),
        }
    }
}

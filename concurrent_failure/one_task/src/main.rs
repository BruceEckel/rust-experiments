#[tokio::main]
async fn main() {
    let handle: tokio::task::JoinHandle<Result<(), &str>> = tokio::spawn(async {
        // Some code here...
        // If everything's fine:
        // Ok(())
        // If something goes wrong:
        Err::<(), &str>("Something went wrong")
    });

    match handle.await {
        Ok(Ok(())) => println!("Task completed successfully."),
        Ok(Err(e)) => println!("Task returned an error: {}", e),
        Err(_) => println!("Task panicked."),
    }
}

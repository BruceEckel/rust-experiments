use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::Mutex;
use tokio::time::sleep;
use FallibleError::*;

#[derive(Error, Debug)]
pub enum FallibleError {
    #[error("V[{0}]")]
    ValueError(i32),
    #[error("T[{0}]")]
    TypeError(i32),
    #[error("A[{0}]")]
    AttributeError(i32),
}

async fn fallible(
    i: i32,
    stdout: Arc<Mutex<()>>,
) -> Result<char, FallibleError> {
    sleep(Duration::from_millis(100)).await;
    {
        // Prevent interleaves to standard output:
        let _lock = stdout.lock().await;
        println!("fallible({})", i);
    } // _lock released

    match i {
        1 => Err(ValueError(i)),
        3 => Err(TypeError(i)),
        5 => Err(AttributeError(i)),
        // 7 => panic!("i:{} panicked!", i),
        _ => {
            sleep(Duration::from_secs(3)).await;
            Ok((b'a' + i as u8) as char)
        }
    }
}

#[tokio::main]
async fn main() {
    // Prevents interleaving std output:
    let stdout = Arc::new(Mutex::new(()));

    let tasks: Vec<_> = (0..8)
        .map(|i| {
            tokio::spawn(fallible(i, stdout.clone()))
        })
        .collect();

    {
        let _lock = stdout.lock().await;
        println!("Tasks created");
    }

    // Run all tasks to completion:
    let results: Vec<_> =
        futures::future::join_all(tasks).await;

    for result in results.iter() {
        // Coroutines finished: no lock needed here
        print!("{:?} => ", result);
        match result {
            Ok(Ok(l)) => println!("Letter: {}", l),
            Ok(Err(e)) => println!("Err: {}", e),
            Err(p) => println!("Panic: {:?}", p),
        }
    }
}

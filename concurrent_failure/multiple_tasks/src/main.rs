use std::panic;
use std::sync::{Arc, Mutex as StdMutex};
use tokio::sync::Mutex as AsyncMutex;

async fn fallible(
    i: usize,
    print_lock: Arc<AsyncMutex<()>>,
) -> Result<char, String> {
    {
        let _lock = print_lock.lock().await;
        println!("fallible({})", i);
    }

    match i {
        5 => panic!("i:{} panicked!", i),
        _ if i % 2 == 0 => {
            Err(format!("Failed({})", i))
        }
        _ => Ok((b'a' + i as u8) as char),
    }
}

#[tokio::main]
async fn main() {
    let print_lock = Arc::new(AsyncMutex::new(()));
    let panic_lock = Arc::new(StdMutex::new(()));

    let plock = panic_lock.clone();
    panic::set_hook(Box::new(move |info| {
        let _lock = plock.lock().unwrap();
        println!("Custom panic: {}", info);
    }));

    let tasks: Vec<_> = (0..10)
        .map(|i| {
            tokio::spawn(fallible(
                i,
                print_lock.clone(),
            ))
        })
        .collect();

    {
        let _lock = print_lock.lock().await;
        println!("Tasks created");
    }

    let results: Vec<_> =
        futures::future::join_all(tasks).await;
    for result in results.iter() {
        let _lock = print_lock.lock().await;
        print!("{:?} => ", result);
        match result {
            Ok(Ok(l)) => println!("Letter: {}", l),
            Ok(Err(e)) => println!("Err: {}", e),
            Err(p) => println!("Panic: {:?}", p),
        }
    }
}

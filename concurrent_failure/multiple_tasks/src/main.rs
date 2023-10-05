use std::sync::Arc;
use tokio::sync::Mutex;

async fn fallible(
    i: usize,
    print_lock: Arc<Mutex<()>>,
) -> Result<char, String> {
    {
        // Prevent interleaves to std output:
        let _lock = print_lock.lock().await;
        println!("fallible({})", i);
    } // _lock released

    match i {
        5 => panic!("i:{} panicked!", i),
        _ if i % 2 == 0 => {
            Err(format!("Failed({})", i))
        }
        // Convert number to char:
        _ => Ok((b'a' + i as u8) as char),
    }
}

#[tokio::main]
async fn main() {
    // Prevents interleaving std output:
    let print_lock = Arc::new(Mutex::new(()));

    let tasks: Vec<_> = (0..10)
        .map(|i| {
            tokio::spawn(fallible(i, print_lock.clone()))
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

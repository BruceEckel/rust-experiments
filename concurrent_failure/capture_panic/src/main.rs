use std::panic::catch_unwind;

#[tokio::main]
async fn main() {
    let tasks: Vec<_> = (0..26).map(|i| {
        tokio::spawn(async move {
            let result = catch_unwind(|| {
                if i == 15 {
                    panic!("Task for i=15 panicked!");
                } else if i % 2 == 0 { // Demonstrate failure:
                    Err::<char, String>(format!("Failed({})", i))
                } else {
                    // Convert number to letter:
                    let letter = (b'a' + i as u8) as char;
                    Ok::<char, String>(letter)
                }
            });
            match result {
                Ok(res) => res,
                Err(_) => Err::<char, String>("A Panic Occurred!".to_string())
            }
        })
    }).collect();

    let results: Vec<_> = futures::future::join_all(tasks).await;

    for result in results.iter() {
        match result {
            Ok(Ok(letter)) => println!("{} [{:?}]", letter, result),
            Ok(Err(e)) => println!("{} [{:?}]", e, result),
            Err(_) => println!("A Panic Occurred in main task!"),
        }
    }
}

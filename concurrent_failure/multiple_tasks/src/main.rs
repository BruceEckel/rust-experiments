// Demonstrate different failures in tasks

#[tokio::main]
async fn main() {
    let tasks: Vec<_> = (0..10)
        .map(|i| {
            tokio::spawn(async move {
                if i == 5 {
                    panic!("i:{} panicked!", i);
                } else if i % 2 == 0 {
                    Err::<char, String>(
                        format!("Failed({})", i)
                    )
                } else {
                    // Convert number to letter:
                    let letter =
                        (b'a' + i as u8) as char;
                    Ok::<char, String>(letter)
                }
            })
        })
        .collect();

    let results: Vec<_> =
        futures::future::join_all(tasks).await;

    for result in results.iter() {
        print!("{:?} => ", result);
        match result {
            Ok(Ok(l)) => println!("Letter: {}", l),
            Ok(Err(e)) => println!("Err: {}", e),
            Err(p) => println!("Panic: {}", p),
        }
    }
}

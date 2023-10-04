#[tokio::main]
async fn main() {
    let tasks: Vec<_> = (0..10)
        .map(|i| {
            tokio::spawn(async move {
                match i {
                    5 => panic!("i:{} panicked!", i),
                    _ if i % 2 == 0 => {
                        Err(format!("Failed({})", i))
                    }
                    _ => Ok((b'a' + i as u8) as char),
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
            Err(p) => println!("Panic: {:?}", p),
        }
    }
}

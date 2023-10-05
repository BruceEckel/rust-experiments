use std::io::{self, Write};

async fn fallible(i: usize) -> Result<char, String> {
    println!("fallible({})", i);
    io::stdout().flush().unwrap();
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
    let tasks: Vec<_> = (0..10)
        .map(|i| tokio::spawn(fallible(i)))
        .collect();
    println!("Tasks created");
    io::stdout().flush().unwrap();

    let results: Vec<_> =
        futures::future::join_all(tasks).await;
    for result in results.iter() {
        print!("{:?} => ", result);
        io::stdout().flush().unwrap();
        match result {
            Ok(Ok(l)) => println!("Letter: {}", l),
            Ok(Err(e)) => println!("Err: {}", e),
            Err(p) => println!("Panic: {:?}", p),
        }
    }
}

// A Mutex protects a shared variable from incorrect modification by multiple threads.
#![allow(unused)]
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    const N: u8 = 10;
    // 'letters' and 'counter' and the tx channel are shared among the threads.
    // Arc stands for "atomic reference count," and it shares memory among threads.
    // The data inside the Arc is protected with a mutex.
    // The data + Mutex is created on the heap and shared through the ref counted Arc:
    let letters = Arc::new(Mutex::new(format!("")));
    let counter = Arc::new(Mutex::new(0));
    let (tx, rx) = channel();
    // Can clone the Sender (tx) but not the Receiver (rx).
    // This is mpsc (multiple-producer single-consumer).

    for _ in 0..N {
        // These reference-count clones are captured by the 'move' on the lambda:
        let (letters, counter, tx) = (letters.clone(), counter.clone(), tx.clone());
        // ^^^ Name-shadowing seems appropriate here. Arc::clone() is more idiomatic.
        println!("{:?} {:?}", counter, letters); // Why no additonal clone()s here?
        thread::spawn(move || {
            // Only this thread can access the shared state when the locks are held.
            // `unwrap()` because acquiring a lock blocks until it is successful.
            // If acquiring a lock fails, panic is appropriate.
            let mut data = letters.lock().unwrap();
            let mut count = counter.lock().unwrap();
            let alpha = *count + 65; // `count` is apparently also a u8.
            *data += &format!("{}", alpha as char); // alpha must be a u8.
            *count += 1;
            tx.send(format!("{count}: {data}"));
            if *count == N { // `N` forces `count` & `counter` to be u8.
                tx.send(format!("done!"));
            }
            // Unlock here when `data` and `count` go out of scope.
        });
    }

    while let Ok(msg) = rx.recv() {
        println!("{msg}");
        if msg.starts_with("done") {
            break;
        };
    }
}

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(format!("hi")).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

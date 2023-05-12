#![allow(unused)]
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel();
    for i in 0..10 {
        // Many producers:
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }

    for _ in 0..10 {
        // Single consumer:
        print!("{} ", rx.recv().unwrap());
    }
}

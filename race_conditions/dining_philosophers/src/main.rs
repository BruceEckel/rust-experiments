// Adapted from
// https://dev.to/valorsoftware/multi-threading-for-impatient-rust-learners-253e

use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::sync::mpsc::{Sender};
use std::sync::mpsc;

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table, sender: &Sender<String>) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        // println!("{} is eating.", self.name);
        sender.send(format!("{} is eating.", self.name)).unwrap();

        let delay = time::Duration::from_millis(250);

        thread::sleep(delay);

        // println!("{} is done eating.", self.name);
        sender.send(format!("{} is done eating.", self.name)).unwrap();
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    let philosophers = vec![
        Philosopher::new("Donald", 0, 1),
        Philosopher::new("Larry", 1, 2),
        Philosopher::new("Mark", 2, 3),
        Philosopher::new("John", 3, 4),
        Philosopher::new("Bruce", 0, 4),
    ];

    let futures: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let table = table.clone();
            let sender = tx.clone();
            thread::spawn(move || {
                p.eat(&table, &sender);
            })
        })
        .collect();

    futures.into_iter().for_each( |f| f.join().unwrap());
    // To make this cleaner:
    // use futures::future::join_all;
    // let futures: Vec<_> = (0..10).map(|_| async_task()).collect();
    // let results: Vec<_> = join_all(futures).await;

    tx.send(format!("Done")).unwrap();

    let mut result: String = format!("");

    for received in rx {
        if received == "Done" {
            break;
        }
        result.push_str(&received);
        result.push_str("\n");
    }
    println!("{}", result);

}

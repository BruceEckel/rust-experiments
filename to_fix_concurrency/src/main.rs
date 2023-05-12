// From https://ochagavia.nl/blog/you-are-holding-it-wrong/
// Doesn't compile -- an exercise to fix it.

use std::thread;

fn main() {
    let mut counter = 0;

    // I haven't seen "scope" before...
    thread::scope(|s| {
        // Add 50000 to the counter in a background thread
        let t1 = s.spawn(|| {
            for _ in 0..50000 {
                counter += 1;
            }
        });

        // Add 50000 to the counter in a background thread
        let t2 = s.spawn(|| {
            for _ in 0..50000 {
                counter += 1;
            }
        });

        // Wait for both threads to finish (I think there's a macro to do this
        // more succinctly)
        t1.join().unwrap();
        t2.join().unwrap();

        println!("Result = {counter}");
    });

}

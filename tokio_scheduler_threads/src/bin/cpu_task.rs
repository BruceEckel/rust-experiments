use std::thread;
use std::time::Instant;

pub fn cpu_bound_task(name: &str, iterations: usize) {
    let current_thread = thread::current();
    println!("{} starts on thread: {:?} (id: {:?})", name, current_thread.name().unwrap_or("[unnamed]"), current_thread.id());
    let start = Instant::now();

    let mut x = 0;
    for _ in 0..iterations {
        x += 1;
    }

    println!("{} ends after {:?}: {}", name, start.elapsed(), x);
}

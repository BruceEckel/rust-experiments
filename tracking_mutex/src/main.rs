use std::sync::{Arc, Mutex};
use std::thread;

struct TrackingMutex<T> {
    inner: Mutex<T>,
}

impl<T> TrackingMutex<T> {
    fn new(data: T) -> Self {
        println!("Creating TrackingMutex");
        Self {
            inner: Mutex::new(data),
        }
    }

    fn lock(&self) -> TrackingMutexGuard<'_, T> {
        println!("Acquiring TrackingMutex.");
        TrackingMutexGuard {
            inner: self.inner.lock().unwrap(),
        }
    }
}

struct TrackingMutexGuard<'a, T> {
    inner: std::sync::MutexGuard<'a, T>,
}

impl<'a, T> Drop for TrackingMutexGuard<'a, T> {
    fn drop(&mut self) {
        println!("TrackingMutex released.");
    }
}

fn main() {
    let data = Arc::new(TrackingMutex::new(42));

    let data_clone = data.clone();
    let handle = thread::spawn(move || {
        let lock = data_clone.lock();
        println!("Inside thread: {}", *lock.inner);
    });

    handle.join().unwrap();

    println!("Main thread finished.");
}

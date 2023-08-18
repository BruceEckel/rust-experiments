use std::sync::Arc;
use std::thread;

fn immutable_data() {
    // Create a vector and wrap it in an Arc
    let data = vec![1, 2, 3, 4, 5];
    let shared_data = Arc::new(data);

    // data[0] += 10;  // No, data is immutable

    // Spawn multiple threads, each accessing the shared data
    let mut handles = vec![];
    for i in 0..5 {
        let data_clone = Arc::clone(&shared_data); // Create a clone of the Arc
        let handle = thread::spawn(move || {
            let val = data_clone[i];
            println!("Thread {}: Value: {}", i, val);
        });
        handles.push(handle);
    }
    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}

use std::sync::Mutex;

fn mutable_data() {
    // Create a vector and wrap it in an Arc with a Mutex
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));

    // Spawn multiple threads, each modifying the shared data
    let mut handles = vec![];
    for i in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut data = data_clone.lock().unwrap(); // Lock the mutex
            data[i] *= 2; // Modify the data
            println!("Thread {}: Modified Value: {}", i, data[i]);
        });
        handles.push(handle);
    }
    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    // Print the final modified data
    let data = data.lock().unwrap();
    println!("Final Data: {:?}", *data);
}

fn _compiler_inference() {
    let data = vec![1, 2, 3, 4, 5];
    let _shared_data = Arc::new(&data);
    let _mutex_shared_data = Arc::new(Mutex::new(&data));
    let x = _shared_data.clone();
    dbg!(x[1]);
    // x[1] += 10;  // cannot borrow data in an `Arc` as mutable
    // dbg!(x[1]);
}

fn main() {
    immutable_data();
    mutable_data();
    _compiler_inference();
}

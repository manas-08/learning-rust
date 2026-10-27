// day 25

// Arc<T> (Atomically Reference Counted) is Rust's thread-safe smart pointer that enables shared ownership of a heap-allocated value across multiple threads.
// It solves the problem of shared ownership by allowing multiple threads to safely access the data.

// Mutex<T> is a mutual exclusion lock. It lets one thread at a time access the protected data through a lock guard. (mutable shared state)

// You must acquire the lock first before accessing the protected value.



// Let's create a complete program where 10 threads each increment the same counter 1,000 times.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {   // Create 10 threads
        // Create another owner of the same shared counter.
        let shared_counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                // Acquire the lock before accessing the protected value.
                let mut count = shared_counter.lock().unwrap();
                *count += 1;
            }
        });
            
        handles.push(handle);
    }

    // Wait for all workers to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
    
}


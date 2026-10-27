use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let tasks = 5;

    // Sequential execution
    let start = Instant::now();

    for _ in 0..tasks {
        thread::sleep(Duration::from_secs(1));
    }

    println!("Sequential: {:?}", start.elapsed());

    // Concurrent execution
    let start = Instant::now();
    let mut handles = Vec::new();

    for _ in 0..tasks {
        let handle = thread::spawn(|| {
            thread::sleep(Duration::from_secs(1));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Concurrent: {:?}", start.elapsed());
}

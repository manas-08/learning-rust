// day 28
// Note: This code benchmarks the performance of checking TCP connections to a list of servers both sequentially and concurrently. It measures the time taken for each approach over multiple runs and calculates the median time for comparison.
// The main idea of measuring performance in real-systems is to run the same operation multiple times and consider the overall performance, rather than relying on a single run which may be affected by various factors such as network latency, system load, etc.


use std::time::{Duration, Instant};

use rust_day28::server_monitor::check_tcp_connection;

fn main() {
    let servers = vec![
        ("example.com", 443),
        ("google.com", 443),
        ("cloudflare.com", 443),
    ];

    let timeout = Duration::from_secs(3);
    let runs = 7;    // Number of runs for benchmarking

    let mut sequential_times = Vec::new();
    let mut concurrent_times = Vec::new();

    for run in 1..=runs {
        // Sequential benchmark
        let start = Instant::now();

        for &(host, port) in &servers {
            let _ = check_tcp_connection(host, port, timeout);
        }

        let sequential_elapsed = start.elapsed();
        sequential_times.push(sequential_elapsed);

        // Concurrent benchmark
        let start = Instant::now();
        let mut handles = Vec::new();

        for &(host, port) in &servers {
            let handle = thread::spawn(move || {
                check_tcp_connection(host, port, timeout)
            });

            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.join().unwrap();   // to finish the thread and get the result
        }

        let concurrent_elapsed = start.elapsed();
        concurrent_times.push(concurrent_elapsed);

        println!(
            "Run {run}: sequential = {:?}, concurrent = {:?}",
            sequential_elapsed, concurrent_elapsed
        );
    }
    
    // Calculate and display median results
    // comparing the performance of sequential and concurrent approaches
    sequential_times.sort();
    concurrent_times.sort();

    let sequential_median = sequential_times[runs / 2];
    let concurrent_median = concurrent_times[runs / 2];

    println!("\n--- Median results ---");
    println!("Sequential: {:?}", sequential_median);
    println!("Concurrent: {:?}", concurrent_median);

    if !concurrent_median.is_zero() {
        println!(
            "Median speedup: {:.2}x",
            sequential_median.as_secs_f64()
                / concurrent_median.as_secs_f64()
        );
    }
}



// The key is that concurrency is a limit we deliberately control—not simply the number of tasks we can spawn.

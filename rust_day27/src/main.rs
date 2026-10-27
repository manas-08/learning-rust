// day 27
// Performance measurement tells us whether our implementation is meeting its goals.

use std::thread;
use std::time::{Duration, Instant};

use rust_day27::server_monitor::check_tcp_connection;

fn main() {
    let servers = vec![
        ("example.com", 443),
        ("google.com", 443),
        ("cloudflare.com", 443),
    ];

    let timeout = Duration::from_secs(3);

    // 1. Sequential checks
    let start = Instant::now();

    for (host, port) in &servers {
        let result = check_tcp_connection(host, *port, timeout);    
        println!("Sequential {host}:{port} -> {result:?}");
    }

    let sequential_time = start.elapsed();

    // 2. Concurrent checks
    let start = Instant::now();
    let mut handles = Vec::new();

    for &(host, port) in &servers { 
        let handle = thread::spawn(move || {
            let result = check_tcp_connection(host, port, timeout);
            (host, port, result)
        });

        handles.push(handle);
    }

    for handle in handles {
        let (host, port, result) = handle.join().unwrap();
        println!("Concurrent {host}:{port} -> {result:?}");
    }

    let concurrent_time = start.elapsed();

    println!("\n--- Performance Comparison ---");
    println!("Sequential: {:?}", sequential_time);
    println!("Concurrent: {:?}", concurrent_time);

    if !concurrent_time.is_zero() {
        println!(
            "Speedup: {:.2}x",
            sequential_time.as_secs_f64()
                / concurrent_time.as_secs_f64()
        );
    }
}
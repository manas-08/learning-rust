use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rust_day26::server_monitor::check_tcp_connection;

fn main() {
    let servers = vec![
        ("example.com".to_string(), 443),
        ("google.com".to_string(), 443),
        ("cloudflare.com".to_string(), 443),
    ];

    let timeout = Duration::from_secs(3);

    // Create one channel.
    let (tx, rx) = mpsc::channel();
    
    let mut handles = Vec::new();

    // Spawn one worker per server.
    for (host, port) in servers {
        let tx = tx.clone();  // why? because we want to send messages from multiple threads to the same receiver. Each thread needs its own sender, so we clone the original sender for each thread.

        let handle = thread::spawn(move || {
            let result = check_tcp_connection(
                &host,
                port,
                timeout,
            );

            // Send the result to the main thread.
            tx.send((host, port, result)).unwrap();
        });

        handles.push(handle);
    }

    // The main thread no longer needs its sender.
    drop(tx);

    // Receive messages until all senders are dropped.
    for (host, port, result) in rx {
        match result {
            Ok(details) => {
                println!(
                    "UP: {}:{} | Address: {} | Latency: {:?}",
                    host,
                    port,
                    details.address,
                    details.elapsed
                );
            }
            Err(error) => {
                println!(
                    "DOWN/ERROR: {}:{} | {:?}",
                    host, port, error
                );
            }
        }
    }

    // Ensure all worker threads have finished.
    for handle in handles {
        handle.join().unwrap();
    }
}
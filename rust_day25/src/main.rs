
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rust_day25::server_monitor::check_tcp_connection;

fn main() {
    let servers = vec![
        ("example.com", 443),
        ("google.com", 443),
        ("cloudflare.com", 443),
    ];

    let healthy_count = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for (host, port) in servers {
        let counter = Arc::clone(&healthy_count); 
        let handle = thread::spawn(move || {
            let result = check_tcp_connection(
                host,
                port,
                Duration::from_secs(3),
            );

            match result {
                Ok(info) => {
                    println!(
                        "[UP] {host}:{port} - {} ms",
                        info.elapsed.as_millis()
                    );

                    let mut count = counter.lock().unwrap();
                    *count += 1;
                }
                Err(error) => {
                    println!("[DOWN] {host}:{port} - {error:?}");
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles { // we are waiting for all the threads to finish before we print the final count of healthy servers.
        handle.join().unwrap();
    }

    let count = healthy_count.lock().unwrap();
    println!("Healthy servers: {}", *count);
}

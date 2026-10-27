// day 24

// Concurrency in Rust
// It allows multiple operations to run simultaneously, making programs faster and more efficient. Rust provides several ways to achieve concurrency, including threads, async/await, and message passing.

use std::thread;
use std::time::Duration;
use rust_day24::server_monitor::check_tcp_connection;


fn main() {
    let servers = vec![
        ("example.com".to_string(), 443),
        ("google.com".to_string(), 443),
        ("cloudflare.com".to_string(), 443),
        ("chatgpt.com".to_string(), 443),
        ("nonexistent.example.com".to_string(), 443),
    ];

    let timeout = Duration::from_secs(1);
    let mut handles = Vec::new();  // Vector to store the handles of the spawned threads.

    // Start a separate worker for each server.
    for (host, port) in servers {
        let handle = thread::spawn(move || {
            let result = check_tcp_connection(
                &host,
                port,
                timeout,
            );

            (host, port, result)              // Return the host, port, and result from the thread.
        });

        handles.push(handle);  // handle is of type JoinHandle<(String, u16, Result<HealthCheckResult, HealthCheckError>)>, which is a handle to the spawned thread.
    }

    // Collect the results.
    for handle in handles {
        match handle.join() {                           // main thread waits for the worker thread to finish and collects the result.
            Ok((host, port, result)) => {
                match result {
                    Ok(info) => {
                        println!(
                            "[UP] {}:{} - {} ms",
                            host,
                            port,
                            info.elapsed.as_millis()
                        );
                    }
                    Err(error) => {
                        println!(
                            "[DOWN] {}:{} - {:?}",
                            host, port, error
                        );
                    }
                }
            }
            Err(_) => {
                eprintln!("A worker thread panicked.");
            }
        }
    }
}

// Notes:
// Remember that spawn() starts the worker thread; join() waits for it to finish!
// The standard thread::spawn() API has a specific contract: the worker may outlive the scope in which it was created (if u don't use join() to wait for it). This means that the worker thread must own all the data it needs, or else it may try to access data that has been dropped, leading to a panic. In this case, we use move to transfer ownership of host and port into the closure, ensuring that the worker thread has its own copy of these values.
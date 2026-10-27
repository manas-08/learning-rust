// day 30

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

use rust_day30::server_monitor::check_tcp_connection;

fn main() {
    let servers = vec![
        ("example.com".to_string(), 443),
        ("google.com".to_string(), 443),
        ("cloudflare.com".to_string(), 443),
        ("iana.org".to_string(), 443),
        ("rust-lang.org".to_string(), 443),
        ("github.com".to_string(), 443),
        ("stackoverflow.com".to_string(), 443),
        ("reddit.com".to_string(), 443),
        ("wikipedia.org".to_string(), 443),
        ("twitter.com".to_string(), 443),
        ("netflix.com".to_string(), 443),
        ("microsoft.com".to_string(), 443),
        ("apple.com".to_string(), 443),
        ("linkedin.com".to_string(), 443),
        ("yahoo.com".to_string(), 443),
        ("bing.com".to_string(), 443),
        ("amazon.com".to_string(), 443),
        ("ebay.com".to_string(), 443),
        ("paypal.com".to_string(), 443),
        ("adobe.com".to_string(), 443),
        ("dropbox.com".to_string(), 443),
        ("slack.com".to_string(), 443),
        ("zoom.us".to_string(), 443),
        ("spotify.com".to_string(), 443),
        ("soundcloud.com".to_string(), 443),
        ("medium.com".to_string(), 443),
        ("quora.com".to_string(), 443),
        ("tumblr.com".to_string(), 443),
        ("flickr.com".to_string(), 443),
        ("pinterest.com".to_string(), 443),
        ("imgur.com".to_string(), 443),
        ("vk.com".to_string(), 443),
        ("jndfub.com".to_string(), 443), // This is a non-existent domain to test error handling.
    ];

    let worker_count = 4; // means only three concurrent connections will be made at a time
    let queue_capacity = 7; // means only four jobs can be queued at a time, the next job will block until a worker is free to take it
    let timeout = Duration::from_secs(3);

    // since the queue is bounded, we can use a sync_channel to block when the queue is full. This prevents the main thread from overwhelming the workers with too many jobs at once. The workers will process jobs at their own pace, and the main thread will wait when the queue is full, ensuring that we don't run out of memory or overwhelm the system with too many concurrent connections.

    // Bounded job queue.
    let (job_tx, job_rx) = mpsc::sync_channel::<(String, u16)>(queue_capacity);

    // Results channel.
    let (result_tx, result_rx) = mpsc::channel();

    // Share the single job receiver across workers.
    let job_rx = Arc::new(Mutex::new(job_rx));
    let mut handles = Vec::new(); // a vector to hold the handles of the worker threads so we can join them later and ensure they have completed before exiting the program.

    // Start workers before submitting jobs.
    for worker_id in 0..worker_count {
        let job_rx = Arc::clone(&job_rx);
        let result_tx = result_tx.clone();

        let handle = thread::spawn(move || {
            loop {
                let job = { job_rx.lock().unwrap().recv() };

                match job {
                    Ok((host, port)) => {
                        println!("Worker {worker_id} checking {host}:{port}");

                        let result = check_tcp_connection(&host, port, timeout);

                        if result_tx.send((host, port, result)).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        handles.push(handle);
    }

    // Submit jobs. send() may block when the queue is full.
    for (host, port) in servers {
        if job_tx.send((host, port)).is_err() {
            eprintln!("Could not submit a server check");
            break;
        }
    }

    // No more jobs will be submitted.
    drop(job_tx);

    // Workers own the remaining result senders.
    drop(result_tx);

    // Receive results until every worker's sender is dropped.
    for (host, port, result) in result_rx {
        match result {
            Ok(details) => {
                println!(
                    "UP   {host}:{port} via {} in {:?}",
                    details.address, details.elapsed
                );
            }
            Err(error) => {
                println!("DOWN {host}:{port} — {error:?}");
            }
        }
    }

    // Wait for workers and report panics.
    for handle in handles {
        if handle.join().is_err() {
            eprintln!("A worker thread panicked");
        }
    }

    println!("All health checks have finished.");
}

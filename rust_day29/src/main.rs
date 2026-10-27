// day 29
// creating a bounded worker pool 

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use rust_day29::server_monitor::check_tcp_connection;

fn main() {
    let servers = vec![
        ("example.com".to_string(), 443),
        ("google.com".to_string(), 443),
        ("cloudflare.com".to_string(), 443),
        ("iana.org".to_string(), 443),
        ("rust-lang.org".to_string(), 443),
    ];

    let worker_count = 3;
    let timeout = Duration::from_secs(3);

    // We'll add the worker pool here.
    // Queue of jobs: (host, port)
    let (job_tx, job_rx) = mpsc::channel::<(String, u16)>();

    // Channel for health-check results
    let (result_tx, result_rx) = mpsc::channel();

    // Share the job receiver between workers
    let job_rx = Arc::new(Mutex::new(job_rx));

    let mut handles = Vec::new();

    for worker_id in 0..worker_count {
        let job_rx = Arc::clone(&job_rx);
        let result_tx = result_tx.clone();   // Clone the result sender for each worker thread. Each worker will have its own sender to send results back to the main thread.

        let handle = thread::spawn(move || {
            loop {
                // Lock only long enough to receive one job.
                let job = {
                    job_rx.lock().unwrap().recv()        // using recv() to receive a job from the job channel. It also removes the job from the channel/queue, so that other workers can pick up the next job (not the same one).
                };

                match job {
                    Ok((host, port)) => {
                        println!(
                            "Worker {worker_id} checking {host}:{port}"
                        );

                        // The mutex is no longer locked during this check.
                        let result =
                            check_tcp_connection(&host, port, timeout);

                        if result_tx.send((host, port, result)).is_err() { // result_tx.send() returns a Result, which we check for errors. If sending the result fails, we break out of the loop and terminate the worker thread.
                            break;
                        }
                    }
                    Err(_) => {
                        // The job channel is closed and empty.
                        break;
                    }
                }
            }
        });

        handles.push(handle);
    }

    // Send every server target to the job queue.
    for (host, port) in servers {
        job_tx.send((host, port)).unwrap();
    }

    // Tell workers there will be no more jobs.
    drop(job_tx);

    // Drop the main thread's result sender.
    // Workers still own their cloned senders.
    drop(result_tx);


    // Receive results until every worker has finished sending.
    for (host, port, result) in result_rx {
        match result {
            Ok(details) => {
                println!(
                    "UP   {host}:{port} via {} in {:?}",
                    details.address,
                    details.elapsed
                );
            }
            Err(error) => {
                println!("DOWN {host}:{port} — {error:?}");
            }
        }
    }

    // Wait for all worker threads to exit.
    for handle in handles {
        if handle.join().is_err() {
            eprintln!("A worker thread panicked");
        }
    }
}



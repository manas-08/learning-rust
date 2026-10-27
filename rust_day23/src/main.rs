use std::time::Duration;

use rust_day23::config::load_config;
use rust_day23::server_monitor::{
    check_tcp_connection,
    HealthCheckError,
};

fn main() {
    let config = match load_config() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Configuration error: {:?}", error);
            return;
        }
    };

    let timeout = Duration::from_secs(3);

    println!(
        "Checking {}:{} (timeout: {:?})",
        config.host, config.port, timeout
    );

    match check_tcp_connection(&config.host, config.port, timeout) {
        Ok(result) => {
            println!("Server is reachable!");
            println!("Connected address: {}", result.address);
            println!("Connection time: {} ms", result.elapsed.as_millis());
            println!("Connection time: {:?}", result.elapsed); 

            // Here, we are measuring only the tcp connection establishment time, not the entire request-response cycle!
        }

        Err(HealthCheckError::Timeout) => {
            eprintln!("Health check timed out.");
        }

        Err(HealthCheckError::ConnectionFailed(error)) => {
            eprintln!("Connection failed: {}", error);
        }

        Err(HealthCheckError::ResolveFailed(error)) => {
            eprintln!("No socket addresses found for this host: {}", error);
        }

        Err(HealthCheckError::NoAddressFound) => {
            eprintln!("No suitable network address we could find for this host to connect to!");
        }
    }
}

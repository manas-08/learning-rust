// day 22

use std::time::Duration;

use rust_day22::config::load_config;
use rust_day22::server_monitor::{
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
        Ok(()) => println!("Server is reachable!"),

        Err(HealthCheckError::Timeout) => {   // this is the case where the connection attempt timed out
            eprintln!("Health check timed out.");
        }

        Err(HealthCheckError::ConnectionFailed(error)) => {     // this is the case where the connection attempt failed for reasons other than timeout
            eprintln!("Connection failed: {}", error);
        }

        Err(HealthCheckError::ResolveFailed(error)) => {   // this is the case where the host could not be resolved to an IP address
            eprintln!("Could not resolve host: {}", error); 
        }

        Err(HealthCheckError::NoAddressFound) => {    // this is the case where the host resolved to no addresses
            eprintln!("No network address found for this host.");   // Why 'eprintln!' instead of println! ? because this is an error message, so it should go to stderr error stream (to report errors), not stdout (to report normal output).
        }
    }
}

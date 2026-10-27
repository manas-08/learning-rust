
use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

#[derive(Debug)]
pub enum HealthCheckError {
    ResolveFailed(io::Error),
    NoAddressFound,
    Timeout,
    ConnectionFailed(io::Error),
}

pub fn check_tcp_connection(
    host: &str,
    port: u16,
    timeout: Duration,
) -> Result<(), HealthCheckError> {   // () means the function returns nothing on success, but it can return a HealthCheckError on failure.
    // Resolve the host and port into socket addresses.
    let addresses = (host, port)
        .to_socket_addrs()
        .map_err(HealthCheckError::ResolveFailed)?;

    let mut last_error = None;  // Variable to store the last error encountered during connection attempts.

    for address in addresses {
        match TcpStream::connect_timeout(&address, timeout) {
            Ok(_connection) => return Ok(()),

            Err(error)
                if error.kind() == io::ErrorKind::TimedOut =>
            {
                return Err(HealthCheckError::Timeout);
            }

            Err(error) => last_error = Some(error), // Store the last error to report if all attempts fail.
        }
    }

    match last_error {
        Some(error) => Err(HealthCheckError::ConnectionFailed(error)),
        None => Err(HealthCheckError::NoAddressFound),
    }
}

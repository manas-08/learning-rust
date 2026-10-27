use std::io;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};
// SocketAddr is a struct that represents a socket address, which is a combination of an IP address and a port number.

#[derive(Debug)]
pub enum HealthCheckError {
    ResolveFailed(io::Error), // when there are no socket addresses found
    NoAddressFound,           // when we could not find any socket address we could connect to!
    Timeout,
    ConnectionFailed(io::Error),
}

#[derive(Debug)]
pub struct HealthCheckResult {
    pub address: SocketAddr,
    pub elapsed: Duration,
}

pub fn check_tcp_connection(
    host: &str,
    port: u16,
    timeout: Duration,
) -> Result<HealthCheckResult, HealthCheckError> {
    let addresses = (host, port)
        .to_socket_addrs()
        .map_err(HealthCheckError::ResolveFailed)?; // when there are no socket addresses found, it will return an error of type HealthCheckError::ResolveFailed

    let mut last_error = None;

    for address in addresses {
        let start = Instant::now(); // Instant is a struct that represents a moment in time

        match TcpStream::connect_timeout(&address, timeout) {
            Ok(_connection) => {
                return Ok(HealthCheckResult {
                    address,
                    elapsed: start.elapsed(), // Return the elapsed time for the successful connection attempt.
                });
            }

            Err(error) if error.kind() == io::ErrorKind::TimedOut => {
                return Err(HealthCheckError::Timeout);
            }

            Err(error) => {
                last_error = Some(error);
            }
        }
    }

    match last_error {
        Some(error) => Err(HealthCheckError::ConnectionFailed(error)),
        None => Err(HealthCheckError::NoAddressFound), // when we could not find any socket address we could connect to, we return a HealthCheckError::NoAddressFound error.
    }
}

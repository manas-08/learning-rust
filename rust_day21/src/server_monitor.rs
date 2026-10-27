use std::net::TcpStream;
use std::io;

pub fn check_tcp_connection(
    host: &str,
    port: u16,
) -> Result<(), io::Error> {
    let _connection = TcpStream::connect((host, port))?;
    // we are doing nothing with the connection now.
    
    Ok(())  // what does () do here inside OK()? In Rust, `()` is called the "unit type" and it represents an empty value or no value at all. It is similar to `void` in other programming languages.
}

struct ServerConfig {
    host: String,
    port: u16,
}

enum ConfigError {
    InvalidHost,
    InvalidPort(String),
    PortZero,
}

fn parse_port(input: &str) -> Result<u16, ConfigError> {
    let port = input
        .parse::<u16>()
        .map_err(|_| ConfigError::InvalidPort(input.to_string()))?;

    if port == 0 {
        return Err(ConfigError::PortZero);
    }

    Ok(port)
}

fn parse_host(input: &str) -> Result<String, ConfigError> {
    if input.is_empty() {
        return Err(ConfigError::InvalidHost);
    }
    Ok(input.to_string())
}

fn create_config(
    host: &str,
    port: &str,
) -> Result<ServerConfig, ConfigError> {
    let host = parse_host(host)?;
    let port = parse_port(port)?;       // create_config() doesn't need to know how parse_port() works. It just needs to know that it returns a Result<u16, ConfigError> and can use the ? operator to propagate errors.

    Ok(ServerConfig {
        host,
        port,
    })
}

fn main() {
    // day 9 
    // We will make error handling more idiomatic and production-oriented

    // map() vs map_err()
    // map()       → transform Ok (success)
    // map_err()   → transform Err [errors]
    match create_config("127.0.0.1", "0") {
    Ok(config) => {
        println!("Server: {}:{}", config.host, config.port);
    }

    Err(error) => {
        // handle the different error types
        match error {
            ConfigError::InvalidPort(port) => {
                println!("Invalid port: {}", port);
            }
            ConfigError::PortZero => {
                println!("Port cannot be zero");
            }
            ConfigError::InvalidHost => {
                println!("Invalid host");
            }
        }
    }
}

}

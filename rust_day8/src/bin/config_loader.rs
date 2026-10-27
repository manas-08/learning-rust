struct ServerConfig {
    host: String,
    port: u16,
}


fn parse_port(input: &str) -> Result<u16, String> {
    match input.parse::<i16>() {
        Ok(port) => {
            if port < 0 {
                Err(String::from("Port number cannot be negative."))
            } else if port == 0 {
                Err(String::from("Port number cannot be zero."))
            } else {
                Ok(port)
            }
        }

        Err(_) => {
            Err(format!("Invalid port number: {}", input))
        }
    }
}

fn create_config(
    host: &str,
    port: &str,
) -> Result<ServerConfig, String> {
    let port = parse_port(port)?;   // ? is used here to propagate error.

    Ok(ServerConfig {
        host: host.to_string(),
        port,
    })
}

fn main() {
    match create_config("127.0.0.1", "avd") {
        Ok(config) => {
            println!("Server: {}:{}", config.host, config.port);
        }

        Err(error) => {
            println!("Configuration error: {}", error);
        }
    }
}
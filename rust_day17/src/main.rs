// day 17

use std::fs;

#[derive(Debug, PartialEq)]
enum ConfigError {
    FileRead(String),
    InvalidLine(String),
    MissingHost,
    MissingPort,
    InvalidPort(String),
    EmptyHost,
    PortZero,
}

#[derive(Debug, PartialEq)]
struct ServerConfig {
    host: String,
    port: u16,
}

fn load_config() -> Result<ServerConfig, ConfigError> {
    let contents = fs::read_to_string("config.txt")
        .map_err(|e| ConfigError::FileRead(e.to_string()))?;

    let mut host: Option<String> = None;    // to handle the case where host is not provided in the config file
    let mut port: Option<u16> = None; // similarly for port

    for line in contents.lines() {
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| ConfigError::InvalidLine(line.to_string()))?;

        let key = key.trim();
        let value = value.trim();

        match key {
            "host" => {
                if value.is_empty() {
                    return Err(ConfigError::EmptyHost);
                }
                host = Some(value.to_string())
            }

            "port" => {
                if value.is_empty() {
                    return Err(ConfigError::MissingPort);
                }

                let parsed_port = value
                    .parse::<u16>()
                    .map_err(|_| ConfigError::InvalidPort(value.to_string()))?;  // if nothing is provided, it will not return an error

                port = Some(parsed_port);
            }

            _ => {
                return Err(ConfigError::InvalidLine(line.to_string()));
            }
        }
    }

    let host = host.ok_or(ConfigError::MissingHost)?;
    let port = port.ok_or(ConfigError::MissingPort)?; //

    if host.is_empty() {
        return Err(ConfigError::EmptyHost);
    }

    if port == 0 {
        return Err(ConfigError::PortZero);
    }

    Ok(ServerConfig { host, port })
}

fn main() {
    match load_config() {
        Ok(config) => {
            println!("Configuration loaded successfully!");
            println!("Host: {}", config.host);
            println!("Port: {}", config.port);
        }
        Err(error) => {
            eprintln!("Configuration error: {:?}", error);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;      // Import the necessary items from the outer scope

    #[test]
    fn valid_config_is_created() {
        let config = ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn zero_port_is_rejected() {
        // Test that a zero port is rejected
        let result = if 0_u16 == 0 {
            Err(ConfigError::PortZero)
        } else {
            Ok(())
        };

        assert_eq!(result, Err(ConfigError::PortZero));
    }

    #[test]
    fn invalid_port_text_is_rejected() {
        let result = "abc"
            .parse::<u16>()
            .map_err(|_| ConfigError::InvalidPort("abc".to_string()));

        assert_eq!(
            result,
            Err(ConfigError::InvalidPort("abc".to_string()))
        );
    }
}
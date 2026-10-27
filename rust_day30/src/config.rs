use std::fs;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    FileRead(String),
    InvalidLine(String),
    MissingHost,
    MissingPort,
    InvalidPort(String),
    EmptyHost,
    PortZero,
    DuplicateKey(String),
}

#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

// separate function to parse the configuration from a string
fn parse_config(contents: &str) -> Result<ServerConfig, ConfigError> {
    let mut host: Option<String> = None;
    let mut port: Option<u16> = None; // similar like 'null' (in java) or 'None' (in python).

    for line in contents.lines() {
        let line = line.trim();
        // Ignore blank lines
        if line.is_empty() {
            continue;
        }

        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| ConfigError::InvalidLine(line.to_string()))?;

        let key = key.trim();
        let value = value.trim();

        match key {
            "host" => {
                if host.is_some() {
                    return Err(ConfigError::DuplicateKey(key.to_string()));
                }
                if value.is_empty() {
                    return Err(ConfigError::EmptyHost);
                }

                host = Some(value.to_string())
            }

            "port" => {
                if port.is_some() {
                    return Err(ConfigError::DuplicateKey(key.to_string()));
                }
                if value.is_empty() {
                    return Err(ConfigError::InvalidPort(value.to_string()));
                }

                let parsed_port = value
                    .parse::<u16>()
                    .map_err(|_| ConfigError::InvalidPort(value.to_string()))?;

                port = Some(parsed_port);
            }

            _ => return Err(ConfigError::InvalidLine(line.to_string())),
        }
    }

    let host = host.ok_or(ConfigError::MissingHost)?;
    let port = port.ok_or(ConfigError::MissingPort)?;

    if port == 0 {
        return Err(ConfigError::PortZero);
    }

    Ok(ServerConfig { host, port })
}

pub fn load_config() -> Result<ServerConfig, ConfigError> {
    load_config_from("config.txt")
}
// load_config_from() lets tests choose a specific file.
pub fn load_config_from(path: &str) -> Result<ServerConfig, ConfigError> {
    let contents = fs::read_to_string(path).map_err(|e| ConfigError::FileRead(e.to_string()))?;

    parse_config(&contents)
}
// This is an example of testability through dependency control: the code no longer forces every caller to use one fixed file path.

// Unit tests for the configuration parsing logic [test module]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_config() {
        let input = "host=localhost\nport=8080";

        let result = parse_config(input);

        assert_eq!(
            result,
            Ok(ServerConfig {
                host: "localhost".to_string(),
                port: 8080,
            })
        );
    }

    #[test]
    fn detects_missing_port() {
        let input = "host=localhost";

        assert_eq!(parse_config(input), Err(ConfigError::MissingPort));
    }

    #[test]
    fn detects_invalid_port() {
        let input = "host=localhost\nport=abc";

        assert_eq!(
            parse_config(input),
            Err(ConfigError::InvalidPort("abc".to_string()))
        );
    }

    #[test]
    fn rejects_port_zero() {
        let input = "host=localhost\nport=0";

        assert_eq!(parse_config(input), Err(ConfigError::PortZero));
    }

    #[test]
    fn detects_duplicate_port() {
        let input = "host=localhost\nport=8080\nport=9090";

        assert_eq!(
            parse_config(input),
            Err(ConfigError::DuplicateKey("port".to_string()))
        );
    }

    #[test]
    fn detects_duplicate_host() {
        let input = "host=localhost\nhost=example.com\nport=8080";

        assert_eq!(
            parse_config(input),
            Err(ConfigError::DuplicateKey("host".to_string()))
        );
    }

    #[test]
    fn rejects_empty_host() {
        let input = "host=\nport=8080";

        assert_eq!(parse_config(input), Err(ConfigError::EmptyHost));
    }

    #[test]
    fn rejects_empty_port() {
        let input = "host=localhost\nport=";

        assert_eq!(
            parse_config(input),
            Err(ConfigError::InvalidPort("".to_string()))
        );
    }

    #[test]
    fn ignores_blank_lines() {
        let input = "\nhost=127.0.0.1\n\nport=8080\n";

        assert_eq!(
            parse_config(input),
            Ok(ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            })
        );
    }
}

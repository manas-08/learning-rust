// day 19
mod config;  // This tells Rust to include the config module.
use crate::config::load_config;   // This imports the load_config function from the config module. So we can use it directly in our main function.
// Think of 'crate' as the root of your project.


fn main() {
    match load_config() {
        Ok(config) => {
            println!("Configuration loaded successfully!");
            println!("Host: {}", config.host);
            println!("Port: {}", config.port);
        }
        Err(error) => {
            println!("Configuration error: {:?}", error);
        }
    }
}


// day 20
// The key distinction: a unit test checks a component; an integration test checks how components interact.

// tests

use rust_day20::config::load_config;
// no longer need to import the config module directly, as it's now part of the public API of your library.

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
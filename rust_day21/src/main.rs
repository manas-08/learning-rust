// day 21
// TCP Networking in Rustt


use rust_day21::config::load_config;
use rust_day21::server_monitor::check_tcp_connection;

fn main() {
    let config = match load_config() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Configuration error: {:?}", error);
            return;
        }
    };

    println!(
        "Checking server {}:{}",
        config.host, config.port
    );

    match check_tcp_connection(&config.host, config.port) {
        Ok(()) => {
            println!("Server is reachable!");
        }
        Err(error) => {
            println!("Server connection failed: {}", error);
        }
    }
}

// Just for testing, u can run --> python -c "import socket; s=socket.socket(); s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1); s.bind(('127.0.0.1', 8080)); s.listen(); print('Listening on port 8080'); s.accept()", in another shell.

// Run 'cargo run' to find that this time the server is reachable, and if you stop the python test server, it will show that the server connection failed.


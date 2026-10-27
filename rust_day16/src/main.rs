// day 16
// file handling

use std::fs;

struct ServerConfig {
    host: String,
    port: u16,
}

fn load_config() -> Result<ServerConfig, String> {
    let contents = fs::read_to_string("config.txt")  // Open the file "config.txt" and read its contents into a string.
        .map_err(|e| format!("Failed to read config: {}", e))?;

    println!("text: {}", std::any::type_name_of_val(&contents)); // 'contents' is a String, so this will print "text: alloc::string::String"

    let mut host = String::new(); // a new empty string to hold the host value
    let mut port = 0; // a new variable to hold the port value


    for line in contents.lines() {  // each line is of type &str [borrowed string slice]
        let parts: Vec<&str> = line.split('=').collect();

        if parts.len() != 2 {
            return Err(format!("Invalid config line: {}", line));
        }

        match parts[0] {    
            "host" => {
                host = parts[1].to_string();
            }

            "port" => {
                port = parts[1]
                    .parse::<u16>()
                    .map_err(|_| "Invalid port".to_string())?;
            }

            _ => {
                return Err(format!("Unknown configuration: {}", parts[0]));
            }
        }
    }
    Ok(ServerConfig { host, port }) // Return the ServerConfig struct with the parsed host and port values.
}

fn main() {
    match load_config() {
        Ok(config) => {
            println!("Host: {}", config.host);
            println!("Port: {}", config.port);
        }

        Err(error) => {
            println!("Configuration error: {}", error);
        }
    }
}


// read_to_string() is a function from the std::fs module that reads the entire contents of a file into a string. It takes a file path as an argument and returns a Result<String, std::io::Error>. 

// lines() is a method on the String type that returns an iterator over the lines of the string. It splits the string at line boundaries and yields each line as a &str.




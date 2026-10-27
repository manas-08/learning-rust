struct Server {
    name: String,
    port: u16,
}

fn server_is_valid(server: &Server) -> bool {   // basically, this code is getting tested through the test cases defined in the tests module. 

    // server name should not be empty and port should be greater than 0
    !server.name.is_empty() && server.port > 0
}

#[cfg(test)]                   
mod tests {                          
    use super::*;  // what does this line do? It allows the tests module to access the items defined in the parent module, such as the Server struct and the server_is_valid function.

    #[test]
    fn test_valid_server() {
        let server = Server {
            name: String::from("MyServer"),
            port: 8080,
        };
        assert!(server_is_valid(&server));
    }

    #[test]
    fn test_invalid_server_empty_name() {
        let server = Server {
            name: String::from(""),
            port: 8080,
        };
        assert!(!server_is_valid(&server));
    }

    #[test]
    fn test_invalid_server_zero_port() {
        let server = Server {
            name: String::from("MyServer"),
            port: 0,
        };
        assert!(!server_is_valid(&server));
    }
}


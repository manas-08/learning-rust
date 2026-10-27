struct Server {
        name: String,
        port: u16,
        healthy: bool,
}

    fn print_server(server: &Server) {
    println!("Server: {}", server.name);
    println!("Port: {}", server.port);
    println!("Healthy: {}", server.healthy);
}
// impl 
    impl Server {
        fn print(&self) {
            println!("Server: {}", self.name);
            println!("Port: {}", self.port);
            println!("Healthy: {}", self.healthy);
        }
    }
    impl Server {
        fn is_healthy(&self) -> bool {
            self.healthy
        }

        fn mark_unhealthy(&mut self) {
            self.healthy = false;
        }
        }

// associated functions 
    impl Server {
    fn new(name: String, port: u16) -> Server {
        Server {
            name,
            port,
            healthy: true,
        }
    }
    }


fn main() {
    // A struct lets you group related data together.
    // struct is a custom data type that lets you name and package together multiple related values that make up a meaningful group.

    let server = Server {    // Create an instance of the struct
        name: String::from("localhost"),
        port: 8080,
        healthy: true,
    };

    print_server(&server);

    // Create a mutable instance of the struct
    let mut server1 = Server {
        name: String::from("server-1"),
        port: 8080,
        healthy: true,
    };

    server1.healthy = false;
    println!("{}", server1.healthy);
    
    server1.print();

    println!("Is server healthy? {}", server.is_healthy());
    server1.mark_unhealthy();
    server1.print();

    let server2 = Server::new(String::from("server-2"), 8080);
    server2.print();
    
}


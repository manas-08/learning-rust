enum ServerStatus {
    Healthy,
    Unhealthy,
    Starting,
}

struct Server {
    name: String,
    port: u16,
    status: ServerStatus,
}

impl Server {
    fn print_status(&self) {
        match self.status {
            ServerStatus::Healthy => println!("Server {} is Healthy", self.name),
            ServerStatus::Unhealthy => println!("Server {} is Unhealthy", self.name),
            ServerStatus::Starting => println!("Server {} is Starting", self.name),
        }
    }
}

fn find_server<'a>(
    name: &str,
    servers: &[&'a Server],
) -> Option<&'a Server> {
    for server in servers {
        if server.name == name {
            return Some(server);
        }
    }
    None
}
// If a function returns a reference that comes from one of its inputs, Rust may need a lifetime annotation to describe that relationship.

fn main() {
    let server1 = Server {
        name: String::from("server-1"),
        port: 8080,
        status: ServerStatus::Healthy,
    };

    let server2 = Server {
        name: String::from("server-2"),
        port: 8081,
        status: ServerStatus::Unhealthy,
    };

    let server3 = Server {
        name: String::from("server-3"),
        port: 8082,
        status: ServerStatus::Starting,
    };

    server1.print_status();
    server2.print_status();
    server3.print_status();


    // vector 
    let servers = vec![&server1, &server2, &server3];
    let ans = find_server("server-2", &servers);
    if let Some(server) = ans {
    println!("Found Server: {}", server.name);
    println!("Port: {}", server.port);

    match server.status {                      // match is like a switch statement in other languages(java, c++).
        ServerStatus::Healthy => println!("Status: Healthy"),
        ServerStatus::Unhealthy => println!("Status: Unhealthy"),
        ServerStatus::Starting => println!("Status: Starting"),
    }
    } else {
        println!("Server not found");
    }
}
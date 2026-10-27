struct Server {
    name: String,
    port: u16,
    healthy: bool,
    request_count: u64,
}

impl Server {
    fn new(name: String, port: u16) -> Server { // associated function
        // ...
        Server {
            name, port, healthy: true, request_count: 0
        }
    }

    fn handle_request(&mut self) {
        self.request_count += 1;
    }

    fn print(&self) {
        println!("Server: {}", self.name);
        println!("Port: {}", self.port);
        println!("Healthy: {}", self.healthy);
        println!("Request Count: {}", self.request_count);
    }

    fn mark_unhealthy(&mut self) {
        self.healthy = false;
    }

    fn is_healthy(&self) -> bool {
        self.healthy
    }
}

fn main() {
    let mut server = Server::new(String::from("localhost"), 8080);

    server.handle_request();
    server.handle_request();
    server.handle_request();
    server.mark_unhealthy();
    server.print();

    let server1 = Server::new(String::from("server-1"), 8080);
    let server2 = Server::new(String::from("server-2"), 8081);
    let server3 = Server::new(String::from("server-3"), 8082);

    let mut vec = Vec::new();
    vec.push(server1);
    vec.push(server2);
    vec.push(server3);
    for server in &vec{
        server.print();
        println!("Is server healthy? {}", server.is_healthy());
    }

}
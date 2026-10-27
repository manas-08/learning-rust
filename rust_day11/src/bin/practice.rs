trait HealthCheck {
    fn check_health(&self) -> bool;
} 

#[derive(Debug)]
enum ServerStatus {
    Healthy,
    Unhealthy,
    Starting,
}            

#[derive(Debug)]
struct Server {         // for custom types, if we want to print them, we need to derive the Debug trait
    name: String,
    port: u16,
    status: ServerStatus,
}

#[derive(Debug)]
struct Database {
    name: String,
    connected: bool,
}

impl HealthCheck for Database {
    fn check_health(&self) -> bool {
        self.connected
    }
}

impl HealthCheck for Server {
    fn check_health(&self) -> bool {
        matches!(self.status, ServerStatus::Healthy)
    }
}

fn monitor<T: HealthCheck>(service: &T) {     // generic method that takes any type T that implements the HealthCheck trait (basically bounded by trait bounds)
    let val:bool = service.check_health();
    if val {
        println!("Component is Healthy: {}", val);
    } else {
        println!("Component is Unhealthy: {}", val);
    }
}

fn main() {
    let server = Server {
    name: String::from("server-1"),
    port: 8080,
    status: ServerStatus::Healthy,
};

    println!("{:#?}", server);
    monitor(&server);

    let database = Database {
    name: String::from("production-db"),
    connected: true,
};

    println!("{:#?}", database);
    monitor(&database);
    // don't consider that server inherits from HealthCheck, it doesn't. It implements the trait. Rust doesn't support inheritance.
    // Server implements a capability called HealthCheck.

}
trait HealthCheck {
    fn check_health(&self) -> bool;
}                    // trait = capability

struct Server {
    name: String,
    port: u16,
}

struct Database {
    name: String,
}

impl HealthCheck for Server {
    fn check_health(&self) -> bool {
        self.port > 0
    }
}

impl HealthCheck for Database {
    fn check_health(&self) -> bool {
        true
    }
}

// impl Server
//     ↓
// Server's own methods

// impl Trait for Server
//     ↓
// Server implements a behavior/interface

fn run_health_check<T: HealthCheck>(service: &T) {       
    println!("Healthy: {}", service.check_health());
}   // T can be any type that implements the HealthCheck trait

// can also be written as:
// fn run_health_check<T>(service: &T)
// where
//     T: HealthCheck,
// {
//     println!("{}", service.check_health());
// }

fn main() {
    // day 11
    let server = Server {
    name: String::from("server-1"),
    port: 8080,
};

    println!("{}", server.check_health());

}

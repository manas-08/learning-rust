trait HealthCheck {
    fn check_health(&self) -> bool;
}

struct Server {
    healthy: bool,
}

struct Database {
    connected: bool,
}

struct Cache {
    available: bool,
}

impl HealthCheck for Server {
    fn check_health(&self) -> bool {
        self.healthy
    }
}

impl HealthCheck for Database {
    fn check_health(&self) -> bool {
        self.connected
    }
}

impl HealthCheck for Cache {
    fn check_health(&self) -> bool {
        self.available
    }
}

fn monitor<T: HealthCheck>(component: &T) {
    println!("Healthy: {}", component.check_health());
}

fn monitor_dynamic(component: &dyn HealthCheck) {
    println!("Healthy: {}", component.check_health());
}

fn main() {

    let server = Server { healthy: true };
    let database = Database { connected: false };
    let cache = Cache { available: true };

    monitor_dynamic(&server);
    monitor_dynamic(&database);

    let components: Vec<Box<dyn HealthCheck>> = vec![
    Box::new(server),
    Box::new(database),
    Box::new(cache),
];

    for component in &components {
    println!("Healthy: {}", component.check_health());
    }

}


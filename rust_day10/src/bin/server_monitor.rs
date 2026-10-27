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

fn count_unhealthy(servers: &[Server]) -> usize {
    servers
    .iter()
    .filter(|Server| matches!(Server.status, ServerStatus::Unhealthy))
    .count()
}

fn count_healthy(servers: &[Server]) -> usize {
    servers
    .iter()
    .filter(|Server| matches!(Server.status, ServerStatus::Healthy))
    .count()
}

fn all_healthy(servers: &[Server]) -> bool {
    servers
    .iter()
    .all(|server| matches!(server.status, ServerStatus::Healthy))
}

fn main() {
    let servers = vec![
    Server {
        name: String::from("server-1"),
        port: 8080,
        status: ServerStatus::Healthy,
    },
    Server {
        name: String::from("server-2"),
        port: 8081,
        status: ServerStatus::Unhealthy,
    },
    Server {
        name: String::from("server-3"),
        port: 8082,
        status: ServerStatus::Healthy,
    },
];

    let healthy_servers: Vec<&String> = servers
    .iter()  // basically borrow the servers vector and iterate over it, so we don't take ownership of the servers vector.
    .filter(|server| matches!(server.status, ServerStatus::Healthy))
    .map(|server| &server.name)  
    .collect();

    println!("Healthy servers: {:?}", healthy_servers);
    println!("Number of healthy servers: {}", count_healthy(&servers));
    println!("Number of unhealthy servers: {}", count_unhealthy(&servers));
    println!("All servers are healthy: {}", all_healthy(&servers));

}
#[derive(Debug)]
struct Server {
    name: String,
    host: String,
    port: u16,
}

fn find_server<'a>(
    name: &str,
    servers: &'a [Server],
) -> Option<&'a Server> {
    for server in servers {
        if server.name == name {
            return Some(server);
        }
    }
    None
}

fn main() {
    let servers = vec![
    Server {
        name: String::from("server-1"),
        host: String::from("127.0.0.1"),
        port: 8080,
    },
    Server {
        name: String::from("server-2"),
        host: String::from("127.0.0.2"),
        port: 8081,
    },
];
    println!("{:#?}", find_server("server-1", &servers));
    println!("{:#?}", find_server("server-2", &servers));
    println!("{:#?}", find_server("server-3", &servers));
}
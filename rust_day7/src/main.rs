#[derive(Debug)]
enum ServerStatus {
    Healthy,
    Unhealthy,
    Starting,
    Stopping,
}
enum RequestResult {
    Success(u16),
    Failed(String),
}

fn print_status(status: ServerStatus) {
    match status {
        ServerStatus::Healthy => {
            println!("Server is healthy");
        }

        ServerStatus::Unhealthy => {
            println!("Server is unhealthy");
        }

        ServerStatus::Starting => {
            println!("Server is starting");
        }

        ServerStatus::Stopping => {
            println!("Server is stopping");
        }
    }
}

fn main() {
    // enum
    // It is used to define a value that can be one of the several variants.
    
    let server_status = ServerStatus::Healthy;
    // The :: means we're accessing a variant associated with the enum.

    print_status(server_status);

    // Option<T> is a built-in enum used to represent an optional value. It acts as a safe alternative to the concept of null.
    // example: Option<i32> can either be Some(i32) or None.

    fn find_server(name: &str) -> Option<u16> {
    if name == "server-1" {
        Some(8080)
    } else {
        None
        }
    }
    
    let result = find_server("server-2");
    
    match result {
        Some(port) => println!("Server runs on port {}", port),
        None => println!("Server not found"),
    }

    // The get method returns an Option<&T> where T is the type of the elements in the vector. Optional referencing.
    let numbers: [i32; 3] = [10, 20, 30];
    match numbers.get(7) {
    Some(value) => println!("Found: {}", value),
    None => println!("Not found"),
    }

    

}

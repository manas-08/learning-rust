use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // day 13
    // Data Structures: Vectors, Strings, HashMaps, Hashsets

    // Use Vec when you simply need a sequence of items.
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec);

    // can do several operations
    vec.push(6);
    vec.pop();

    // u can access elements by index in o(1) time

    // if u want to search for an element in a vector, it requires o(n) time complexity.


    // HashMap 
    // u can access elements in o(1) time, inserting and deleting elements is also o(1) time complexity.
    let mut servers = HashMap::new();

    // can do several operations
    servers.insert("server-1", 8080);
    servers.insert("server-2", 8081);
    servers.insert("server-3", 8082);
    println!("{:?}", servers);

    let port = servers.get("server-1");             // .get() returns an Option<&V> type. Reliable programming
    println!("Port for server-1: {:?}", port);

    let mut ips = HashSet::new();         // for storing unique values, HashSet is used. It does not allow duplicate values.

    ips.insert("192.168.1.10");
    ips.insert("192.168.1.20");
    ips.insert("192.168.1.10");
    println!("{:?}", ips);

    // entry() method is used to check if a key exists in the HashMap. If it does not exist, it will insert the key with the provided value.
    servers.entry("server-4").or_insert(8083);
}

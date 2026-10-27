use std::collections::HashMap;
fn main() {
    let failures = vec![
    "web-1",
    "db-1",
    "web-1",
    "cache-1",
    "db-1",
    "web-1",
];

    let mut hashmap = HashMap::new();
    for failure in failures {
        let count = hashmap.entry(failure).or_insert(0); // .or_insert() method is used to check if a key exists in the HashMap. If it does not exist, it will insert the key with the provided value. It returns a mutable reference to the value corresponding to the key. If the key already exists, it returns a mutable reference to the existing value.
        *count += 1;
    }

    println!("{:#?}", hashmap);
}
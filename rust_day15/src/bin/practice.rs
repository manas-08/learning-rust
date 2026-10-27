struct Server {
    name: String,
    host: String,
    port: u16,
}

fn is_valid_port(port: u16) -> bool{
    port > 0
}


#[cfg(test)]                     // Compile this module only when running tests.

mod tests {                          // test cases for is_valid_port function
    use super::*;

    #[test]
fn test_valid_port() {
    assert!(is_valid_port(1));
    assert!(is_valid_port(8080));
    assert!(is_valid_port(65535));
}

#[test]
fn test_invalid_port() {
    assert!(!is_valid_port(0));
}
}

// When u run tests, the Rust compiler will compile the code in the tests module and run the test cases defined within it. 
// The #[cfg(test)] attribute ensures that this module is only included during testing, so it won't affect the normal compilation of your program.




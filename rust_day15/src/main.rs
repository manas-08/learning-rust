fn is_valid_port(port: u16) -> bool {
    port > 0
}

// 'cargo test' -> builds your application code so the tests can use it, but it runs the tests, not your normal application flow.

#[cfg(test)]                     // Compile this module only when running tests.
mod tests {                          // test cases for is_valid_port function
    use super::*;

    #[test]
    fn test_valid_port() {
        assert!(is_valid_port(8080));
    }

    #[test]
    fn test_invalid_port() {
        assert!(!is_valid_port(0));
    }
}


// assert! -> checks whether a condition is true, and if it is not, the test will fail. 
// asserteq! -> checks whether two values are equal, and if they are not, the test will fail.
// assert_ne! -> checks whether two values are not equal, and if they are equal, the test will fail.
// #[cfg(test)] -> This attribute tells the Rust compiler to compile the following module only when running tests. It allows you to include test code in your source files without affecting the normal compilation of your program.




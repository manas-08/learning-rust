use rust_day20::config::{load_config_from, ConfigError, ServerConfig};
use std::fs;
use std::env;


#[test]
fn loads_valid_config_from_file() {
    let path = env::temp_dir().join(format!(
        "server_monitor_valid_{}.txt",   // {} will contain the process ID, making the filename unique for each test run.
        std::process::id()
    ));

    fs::write(&path, "host=localhost\nport=8080")
        .expect("failed to write test config");

    let result = load_config_from(path.to_str().unwrap());  // unwrap the path to get a &str

    let _ = fs::remove_file(&path); // Why didn't we just write right hand portion, why declare a variable? Because we want to ignore the result of remove_file, but we still want to call it. If we just wrote fs::remove_file(&path);, the compiler would warn us that we're ignoring the Result returned by remove_file. By assigning it to _, we're explicitly saying "I know this returns a Result, but I don't care about it."

    assert_eq!(
        result,
        Ok(ServerConfig {
            host: "localhost".to_string(),
            port: 8080,
        })
    );
}

#[test]
fn rejects_invalid_port_from_file() {
    let path = env::temp_dir().join(format!(
        "server_monitor_invalid_{}.txt",
        std::process::id()   // Why do we use process::id() here? Because we want to create a unique filename for each test run. If we used a fixed filename, and two tests ran at the same time (or if a previous test run left the file behind), we could get a race condition or a conflict. By including the process ID in the filename, we ensure that each test run gets its own unique file.
    ));

    fs::write(&path, "host=localhost\nport=abc")
        .expect("failed to write test config");

    let result = load_config_from(path.to_str().unwrap());

    let _ = fs::remove_file(&path);

    assert_eq!(
        result,
        Err(ConfigError::InvalidPort("abc".to_string()))
    );
}

// In these tests, we are actually involving two components: the file system (to read the configuration file) and the configuration parsing logic. This is why these are considered integration tests rather than unit tests.

// -- to run only the integration tests, use cargo test --test config_integration!
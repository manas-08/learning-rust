fn main() {
    let logs = vec![
        "INFO User logged in",
        "ERROR Database connection failed",
        "INFO Request completed",
        "WARNING High memory usage",
        "ERROR Request timeout",
    ];

    for log in &logs {
        println!("{}", log);
    }

    let error_count = count_errors(&logs);
    println!("Number of errors: {}", error_count);
}

fn count_errors(logs: &[&str]) -> usize {
    
    let mut count = 0;
    for log in logs {
            if log.starts_with("ERROR") {
                count += 1;
            }
    }
    count                                 // count is returned as the number of errors in the logs
}

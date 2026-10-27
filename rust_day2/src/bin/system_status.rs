fn main() {
    let cpu_usage = [75, 80, 90, 60, 50];
    for usage in cpu_usage {
        
    let status = if usage < 50 {
        "Normal"
    } else if usage < 80 {
        "High"
    } else {
        "Critical"
    };

    match status {
        "Normal" => println!("System status: Normal"),
        "High" => println!("System status: High"),
        "Critical" => println!("System status: Critical"),
        _ => println!("Unknown status"),
    }
}
}
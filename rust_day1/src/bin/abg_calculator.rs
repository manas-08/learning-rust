fn main() {
    let a = 5.0;
    let b = 10.0;

    println!("Average of {} and {} is {}", a, b, avg(a, b));
}

fn avg(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}
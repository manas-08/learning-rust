fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    let celsius = 30.0;

    let fahrenheit = celsius_to_fahrenheit(celsius);

    println!("{}°C = {}°F", celsius, fahrenheit);

    let fahrenheit = 86.0;

    let celsius = fahrenheit_to_celsius(fahrenheit);

    println!("{}°F = {}°C", fahrenheit, celsius);
}
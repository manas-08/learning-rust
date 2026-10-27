fn main() {
    let numbers = [10, 25, 42, 7, 100];

    for number in numbers {
        if number % 2 == 0 {
            println!("{} is even", number);
        } else {
            println!("{} is odd", number);
        }
    }
}
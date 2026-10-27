fn calculate_length(text: &String) -> usize {
    text.len()
}
fn add_suffix(text: &mut String) {
    text.push_str(" Programming");
}

fn main() {
    let mut message = String::from("Hello Rust");

    let length1 = calculate_length(&message); // borrwing the value of message to calculate its length using '&' operator.
    println!("Length of message: {}", length1);

    add_suffix(&mut message); // mutable borrowing to add a suffix to the message using '&mut' operator.
    
    let length2 = calculate_length(&message); // borrwing the value of message to calculate its length using '&' operator.
    println!("Length of message now: {}", length2);
}


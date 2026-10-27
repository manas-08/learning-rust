fn main() {

    let mut s = String::from("hello");
    s.push('!');   // for a single character
    println!("{}", s);

    s.push_str(" world"); // for multiple characters to a string
    println!("{}", s);

    // another way to create a string
    let s1 = "Manas".to_string(); // still creating a string type

    let s2: &str = "Manas"; // this is a string slice, not a string type; s2 is a reference to a string literal


    // len() method returns the length of a string in bytes, and string in rust store utf-8 encoded characters, so the length of a string in rust is not equal to the number of characters in the string.
    let message = String::from("नमस्ते");
    println!("Length of the string in bytes: {}", message.len()); // returns 18, because each character in the string is represented by 3 bytes in utf-8 encoding
    // Thus, Rust doesn't allow normal indexing into a String.

    let hello = &message[..6]; // this will work because the first 6 bytes represent the first two characters in the string
    println!("First two characters of the string: {}", hello);
    // here, hello does not own the string, it just borrows a portion of the original string owned by message variable.

    // also str[..] this means entire string. 


    // Vec<T> is a growable array

    let mut v = Vec::new(); // creates an empty vector
    v.push(1); // adds an element to the end of the vector
    v.push(2);
    v.push(3);

    println!("Vector: {:?}", v); // prints the vector

    let val = v.get(1); // returns an Option<&T> type, which is Some(&2) in this case
    println!("Value at index 1: {:?}", val);


    let mut vector = vec![1, 2, 3, 4, 5]; 
    for v in &mut vector { 
        *v *= 2;
        println!("{}", v);
    }
}
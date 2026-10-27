fn main() {
    let a = 10;
    let b = a;
    println!("a: {}, b: {}", a, b); // both a and b can be used because i32 is a Copy type.
    // Passing ownership into a function can make the original variable unusable. (in case of a String)
}


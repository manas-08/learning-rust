// fn main() {
//     let name = "Manas";

//     println!("Hello, {}", name);
// }

fn main() {
    let mut age = 22;    // 'mut' makes the variable mutable

    age = 23; 

    println!("Age: {}", age);
    println!("Area: {}", area_rectangle(5.0, 10.0));
    println!("Sum: {}", add(5, 10));
    // func();
}


// name is a variable that holds the string "Manas". Variables in Rust are immutable by default.

// rust is a statically typed language, which means that the type of a variable must be known at compile time.
fn func() {
    let age: i32 = 22;
    let height: f64 = 5.9;
    let is_student: bool = true;
    let grade: char = 'A';

    println!("Age: {}", age);
    println!("Height: {}", height);
    println!("Student: {}", is_student);
    println!("Grade: {}", grade);
}
//Static typing + Type inference: Rust can infer the type of a variable based on the value assigned to it. For example, if you assign an integer value to a variable, Rust will infer that the variable is of type i32. However, you can also explicitly specify the type of a variable using a type annotation.\\



fn add(a: i32, b:i32) -> i32 {
    a + b
}

fn print_number(number: i32) {
    println!("{}", number);
}

fn print_name(name: String) {
    println!("{}", name);
}

// functions can return anything.
fn main() {
    let x = 10;

    print_number(x);   // here x is copied to the function, so it can still be used after the function call.
    println!("{}", x);

    let name = String::from("Manas"); 

    //print_name(name);  // here name is moved to the function, so it cannot be used after the function call because ownership is transferred to the function.

    //println!("{}", name);

    let name2: &str = "Manas"; // here name2 is a string slice, so it can be used after the function call because it is a reference to the string literal.
    print_name(name2.to_string());
    println!("{}", name2);

    let name3 = name;
    //println!("{}", name);  // name does not work here because it was moved to name3, so it cannot be used after the move.


    // cloning 
    let x = String::from("hello");

    let y = x.clone();

    println!("{}", x);
    println!("{}", y);
    // cloning is expensive because it creates a new copy of the data, so it should be used only when necessary.
}

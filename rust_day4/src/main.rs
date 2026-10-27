fn main() {
    // day 4

    // Two memory areas: Stack and Heap
    // stack: fixed size variables store hote hai, While heap: dynamic size like String rhti hai!


    // Ownership is a set of rules that governs how a Rust program manages memory.
    // Ownership rules:
    // Rule 1: Every value has an owner.
    // Rule 2: There can only be one owner at a time.
    // Rule 3: When the owner goes out of scope, the value is dropped.

    let x = 10;
    // scope of x is the main function while the scope of y is the inner block
    {
        let y = 20;

        println!("{}", y);
    }

    println!("{}", x);


    let s1 = String::from("hello");
    let s2 = s1; // ownership of s1 is moved to s2, so s1 is no longer valid
    // println!("{}", s1);
    println!("{}", s2);

    // Move vs Copy
    // Simple stack-like values
    //      ↓
    // often Copy

    // Heap-owning values
    //      ↓
    // often Move


    // Borrowing
    // Borrowing is a way to let multiple parts of your code access the same data without taking ownership of it.
    
    let message = String::from("Hello Rust");
    process(&message);      // '&' is used to borrow the value of message.
    println!("Still available: {}", message);

        // mutable borrowing
    let mut name = String::from("John");
    println!("Original name: {}", name);
    change_name(&mut name); // '&mut' is used to borrow the value of name but mutably, so we can change it.
    println!("Changed name: {}", name);


    // important: we can have multiple immutable references or one mutable reference to a value at a time, but not both. This is to prevent data races at compile time.

    // example of data race:
    let mut data = String::from("Data");
    let r1 = &data; // immutable reference
    let r2 = &data; // immutable reference
    // let r3 = &mut data; // mutable reference, this will cause a compile error because we already have immutable references to data.
    println!("r1: {}, r2: {}", r1, r2);
}

fn process(data: &String) {      // we can use &str as well. &str is a borrowed string slice!
    println!("Processing: {}", data);
}

fn change_name(name: &mut String) {
    name.push_str(" Smith");
}
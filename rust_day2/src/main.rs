fn main() {
    let a: i8 = 10;
let b: i16 = 10;
let c: i32 = 10;
let d: i64 = 10;
let e: i128 = 10;


let numbers = [10, 20, 30];
let length: usize = numbers.len();
println!("Length of numbers: {}", length);
// usize is an unsigned integer type that is the same size as a pointer

let person = ("Manas", 22, true);
println!("Name: {}, Age: {}, Is Student: {}", person.0, person.1, person.2);
// tuples are useful when you want to return multiple values from a function or when you want to group together related values that have different types


let arr: [i32; 5] = [1, 2, 3, 4, 5];  // array contains value of same type and fixed size
println!("Array 1st element: {}", arr[0]);
//println!("Array length: {}", arr[10]); // performs bounds checking and will panic if the index is out of bounds


// if else
let age = 20;

// if is actually an expression in Rust, which means it returns a value.
let status = if age >= 18 {
    "Adult"
} else {
    "Minor"
};
println!("{}", status);

// match statement
let status = 201;

match status {
    200 => println!("Success"),
    404 => println!("Not Found"),
    500 => println!("Server Error"),
    _ => println!("Other status"),     // _ means anything else
}

// loops
let mut ct = 0;
loop {                              // loop keyword is used to create an infinite loop
    println!("Hello!");
    ct+=1;
    if ct == 10 {break;}
}

// while loop
let mut ct = 0;
while ct < 10 {
    println!("Hello!");
    ct+=1;  
}


// for 
for i in 0..10 {                  // 0..10 is a range that includes 0 and excludes 10
    println!("Hello!");}

for num in numbers {
    println!("Number: {}", num);
}
}



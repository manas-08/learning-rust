fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn calculate() -> Result<i32, String> {
    let result = divide(10, 2)?; // The ? operator is used to propagate errors. If divide returns an Err, calculate will return that Err immediately. Basically, the error is propagated up the call stack. (to the caller of calculate i.e., main function in this case)

    Ok(result + 10)
}

fn main() {
    // day 8

    // enum Result<T, E> {
    // Ok(T),
    // Err(E),
    //}
    // T = successful result
    // E = error type

    // Result is a built-in enum used for error handling. It represents either a success (Ok) or an error (Err). It is a generic type that can hold any type of value for both the success and error cases.

    let res = divide(10, 1);
    match res {
        Ok(value) => {
            println!("Result: {}", value);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    let val = calculate();
    println!("Calculation result: {:?}", val);

    // Option vs Result
    // Option handles absence. Result handles failure.

}

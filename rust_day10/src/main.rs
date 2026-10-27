fn main() {
    // day 10

    // iterator -> something that gives you values one at a time.
    // Doesn't create another collection, just provides a way to process elements one at a time.

    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("The number is: {}", number);
    }   // this is how iterators work. Here, numbers are borrowed, so numbers is still valid after the for loop.

    for number in numbers.into_iter() {
        println!("The number is: {}", number);
    }    // numbers is moved into the for loop, so numbers is no longer valid after that point.

    //println!("The numbers are: {:?}", numbers); // can't be printed.

    
    // Closures -> anonymous functions; useful when passing small pieces of logic into functions.

    let double = |x| x * 2;
    println!("Double is: {}", double(2));

    let num = vec![1,2,3];

    let doubled: Vec<i32> = num.iter()
        .map(|x| x*2)
        .collect(); // collect() is used to turn the iterator back into a collection because iterators are lazy and don't do anything until you call a method like collect() or for_each().

    println!("The doubled numbers are: {:?}", doubled);

    let result: Vec<i32> = num
    .iter()
    .filter(|x| **x % 2 == 0)
    .map(|x| x * 2)
    .collect();   
    println!("The doubled even numbers are: {:?}", result);

    
}

fn main() {
    //day 14

    // Today's motto: Who owns the text, who is borrowing it, and why Rust has both String and &str.

    let mut name = String::from("server");   // String -> owned text, heap allocated, mutable
    name.push_str("-1");

    println!("Hello, {}!", name); //The String owns the memory containing the text.

    let name2: &str = "server-2"; // &str -> borrowed text, immutable, stack allocated
    // the string literal is stored in the program's binary and has a fixed lifetime.
    println!("Hello, {}!", name2);

    let ref_name = &name; // string 'name' is referenced by 'ref_name'.
    let name_ref = name.as_str(); // string 'name' is converted to a string slice '&str' and assigned to 'name_ref'. ANOTHER way to borrow the string.

    //name2.push_str("-3"); // This line will cause a compile-time error because 'name2' is an immutable reference to a string literal, and you cannot modify it.

    let owned_name2 = name2.to_string(); // Convert the borrowed string slice '&str' to an owned String.
    // or let owned_name = String::from(name);

    // when u need to read; borrow

    // Also, rust support utf-8, so you can use emojis in your strings!!!


    let text1 = String::from("नमस्ते");
    for c in text1.chars() {              // chars() returns an iterator for unicode scalar values (characters).
        print!("{}", c);
    }
    println!("\nLength of the string in bytes: {}", text1.len()); // Length in bytes
    println!("Length of the string in characters: {}", text1.chars().count()); // Length

    // string sliciing

    let text2 = String::from("hello");
    let part = &text2[0..2];
    // this is based on byte indices, not character positions.
    println!("Part of the string: {}", part);

    let part2 = &text1[0..3]; // This will not panic at runtime. Depending on the characters, it may not be valid UTF-8 slicing. In this case, we know "न" is a 3 byte character. Otherwise, it can panic at runtime.

    println!("Part of the string: {}", part2);

    // Note:

    // Use String when you need to own the text. Use &str when you only need to borrow/read the text.

    // indexing is not allowed on String or &str because of the variable byte length of UTF-8 characters. Instead, you can use methods like chars() to iterate over characters or use slicing with caution.
    // slicing is allowed on String and &str, but you need to be careful with the byte indices to avoid slicing in the middle of a character.



}

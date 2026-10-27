fn main() {
    // day 12

    // Static dispatch: The compiler knows the type of the object at compile time, so it can generate optimized code for that specific type. This is faster but less flexible.
    // Dynamic dispatch: The compiler doesn't know the type of the object at compile time, so it uses a vtable (virtual method table) to look up the method to call at runtime. This is more flexible but slower.
    // In Rust, static dispatch is achieved through generics and traits, while dynamic dispatch is achieved through trait objects (using `dyn` keyword).
    
// Static
// ------
// "I know what you are."

// Dynamic
// -------
// "I don't know what you are,
// but I know what you can do."

// Box<T> -> means roughly: Put T on the heap and give me ownership through a pointer.

// Box<dyn Trait -> Give me an owned trait object.

/// In Rust, the impl (implementation) keyword is primarily used to add functionality to types. Because Rust doesn't use traditional classes, impl blocks are how you define methods, associated functions, and trait implementations for structs and enums.

fn get_server() -> impl HealthCheck {
    Server {
        healthy: true,
    }
}       // impl Trait -> Return some concrete type implementing the trait without exposing the concrete type in the API.

fn get_server() -> Box<dyn HealthCheck> {
    Box::new(Server {
        healthy: true,
    })
}   // Box<dyn Trait> -> Return a trait object, allowing for dynamic dispatch and type erasure, enabling polymorphism.


}

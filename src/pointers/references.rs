/*
Normal Pointers (References)
Types: Immutable (&T) and Mutable (&mut T).
Safety: Guaranteed by Rust's borrowing rules; ensures no data races or invalid memory access.
Automatic Dereferencing: Rust automatically dereferences these pointers when methods are called or fields are accessed.
Usage:
Immutable references (&T) are used for read-only access to data without taking ownership.
Mutable references (&mut T) are used for modifying data without taking ownership. Only one mutable reference is allowed at a time to prevent data races.
Function Parameters: Often used to pass data to functions without transferring ownership, allowing for efficient data access and modification.

Explicit dereferencing (*) is required when you need to work directly with the value a reference points to, especially for arithmetic operations, modifications, or when you're passing a reference to a function expecting a value.


Raw Pointers
Types: Immutable (*const T) and Mutable (*mut T).
Safety: Considered unsafe; they can be null, dangling, or point to invalid memory.
Explicit Dereferencing: Must be explicitly dereferenced using the * operator within an unsafe block.
Usage:
Often used for low-level memory manipulation, interfacing with C code, or when building safe abstractions manually.
Requires careful handling due to the lack of safety guarantees.
Function Parameters: Can be passed to functions, but operations on them must be enclosed in unsafe blocks. Used in scenarios where Rust's safety guarantees need to be bypassed for greater control.
Summary
Normal pointers are the default in Rust, providing safety and ease of use. They adhere to Rust's strict borrowing rules, ensuring memory safety.
Raw pointers offer more control and are used for specific low-level tasks, but they come with the risk of unsafe operations and require explicit management of safety concerns. Operations involving raw pointers must be enclosed in unsafe blocks to acknowledge and handle the associated risks.
 */

fn print_value(val: &i32) {
    println!("Value: {}", val);
}

fn modify_value(val: &mut i32) {
    *val += 1;
}

pub fn run() {
    let x = 10;
    // modify_value(&mut x);
    print_value(&x);
}

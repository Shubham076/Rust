/*
Ownership Rules
Each value in Rust has an owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.(the memory is automatically returned once the variable that owns it goes out of scope)

References rules:
At any given time, you can have either one mutable reference or any number of immutable references in a particular scope.
References must always be valid.

Passing a variable to a function will transfer ownership (in case of primitives a copy is sent as size is know at compile time)
Returning values can also transfer ownership.

& reference and borrowing operator in rust
A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
but we can have mutable references

Mutable references to have one big restriction: if you have mutable reference to a value, you can have no other references to that value
if you still have requirement for multiple mutable references we can create a new scope

We also cannot have a mutable reference while we have an immutable one to the same value.
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);

A reference’s scope starts from where it is introduced and continues through the last time that reference is used
solution:
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

 */

fn print_val(x: &String) {
    println!("{}", x)
}
pub fn run() {
    let x = String::from("name");
    print_val(&x);
    print_val(&x);
}
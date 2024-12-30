

pub fn main() {
    println!("hello world");

    // formatting
    println!("My name is {} ", "Shubham");

    // positional argument
    println!("{0} has a {2} and {0} has a {1}", "Alex", "Cat", "Dog");

    //named arguments
    println!("{name}, {surname}", name="shubham", surname="dogra");

    // binary and octal and hex conversion printing traits
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 5, 5, 5);

    //debug
    println!("Array: {:?}", [1, 2, 3]);
}
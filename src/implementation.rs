/*
Implement some functionality for a type.

The impl keyword is primarily used to define implementations on types. Inherent implementations are standalone, while trait implementations are used to implement traits for types, or other traits.

Functions and consts can both be defined in an implementation. A function defined in an impl block can be standalone, meaning it would be called like Foo::bar(). If the function takes self, &self, or &mut self as its first argument, it can also be called using method-call syntax, a familiar feature to any object oriented programmer, like foo.bar().

 */
struct Temperature {
    val:f32,
}

impl Temperature {
    fn print_temperature(&self) {
        println!("{}", self.val)
    }
}

pub fn run() {
    let temp = Temperature {
        val: 22.3
    };
    temp.print_temperature()
}
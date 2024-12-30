//A tuple is a collection of values of different types. Tuples are constructed using parentheses (), and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the types of its members. Functions can use tuples to return multiple values, as tuples can hold any number of values.

pub fn run() {
    let x = (1, "name", 2.0);
    println!("{}", x.0);
    println!("{}", x.1);
    println!("{}", x.2);
}
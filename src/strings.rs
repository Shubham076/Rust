/*

String literals / slice (&str) are used when the value of a string is known at compile time.
String literals are static by default. This means that string literals are
guaranteed to be valid for the duration of the entire program
String literals are immutable by default

The String object type is provided in Standard Library. Unlike string literal,
the string object type is not a part of the core language. It is defined as public structure
in standard library pub struct String. String is a growable collection. It is mutable and UTF-8 encoded type.
 */

fn run(){
    let company: &str = "TutorialsPoint";
    let location: &str = "Hyderabad";
    println!("company is : {} location :{}",company,location);


    let empty_string = String::new();
    println!("length is {}",empty_string.len());

    let content_string = String::from("TutorialsPoint");
    println!("length is {}",content_string.len());

    // format macro use to build string
    let name = format!("Hi I'm is {}", "shubham");

    // push and push string
    let mut name = String::from("hell");
    name.push('o');
    name.push_str(" world!")
}

struct Person {
    name: i32,
    age: i32,
}

pub fn run() {
    let p = Person {
        name: 12,
        age: 10,
    };
    println!("Person with name {} and age {}", p.name, p.age)
}
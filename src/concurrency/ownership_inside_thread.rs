use std::thread;

struct Test {
    name: String, // Owned String, not a static reference
}

pub fn run() {
    let a = Test {
        name: "Shubham".to_string(),
    };

    let handle = thread::spawn(move || {
        println!("a: {}", a.name);
    });

    handle.join().unwrap();

    // // Error: value borrowed here after move
    // println!("{}", a.name);
}

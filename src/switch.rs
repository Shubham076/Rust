//match is checked by the compiler
pub fn run() {
    let val = 20;
    match val {
        10 => println!("10"),
        11 => println!("11"),
        _ => println!("default")
    }
}

pub fn run() {
    let a = 100;
    if a > 10 {
        println!("a is greater than 10")
    } else if a > 2 && a < 10 {
        println!("a is greater than 2 less than 10")
    } else {
        println!("a is smaller than 2")
    }
}
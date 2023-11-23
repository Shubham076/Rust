/*
Rust’s standard library features for input and output are organized around two traits −
*/


mod fileio;

use std::io::Write;

fn run(){
    let mut line = String::new();
    println!("Enter your name :");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line);
    println!("no of bytes read , {}", b1);

    let b1 = std::io::stdout().write("Tutorials ".as_bytes()).unwrap();
    let b2 = std::io::stdout().write(String::from("Point").as_bytes()).unwrap();
    std::io::stdout().write(format!("\nbytes written {}",(b1+b2)).as_bytes()).unwrap();

    let cmd_line = std::env::args();
    println!("No of elements in arguments is :{}",cmd_line.len());
    //print total number of values passed
    for arg in cmd_line {
        println!("[{}]",arg); //print all values passed
    }

}



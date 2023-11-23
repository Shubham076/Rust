use std::thread;
use std::sync::mpsc;

fn main() {

    let (tx, rx) = mpsc::channel();
    // clone the sender
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {

        // send from the regular transmitter
        tx.send("Hello").unwrap();
    });

    thread::spawn(move || {

        // send from the cloned transmitter
        tx1.send("there").unwrap();
    });

    let a:&str = rx.recv().unwrap();
    let b:&str = rx.recv().unwrap();
    println!("{} {}", a, b);
}
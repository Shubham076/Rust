use std::thread;
use std::sync::mpsc;

// mpsc multiple producer single consumer
pub fn run() {

    // create send/receiver vars
    // to move data through channel
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {

        // value to be sent
        let a:i32 = 5;

        // the send() function will send
        // a value to the receiver (rx)
        tx.send(a).unwrap();
    });

    let b:i32 = rx.recv().unwrap();

    println!("Value a from extra thread: {}", b);
}
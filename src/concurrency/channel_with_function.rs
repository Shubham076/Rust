use std::thread;
use std::sync::mpsc;

// sends an int32 value
// from a channel
fn logger(a: mpsc::Sender<i32>) {

    // send value
    a.send(5).unwrap();
}

pub fn run() {

    // create send/receiver vars
    // to move data through channel
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {

        // call function with
        // sender as parameter
        logger(tx);
    });

    handle.join().unwrap();

    // receive value
    let b = rx.recv().unwrap();
    println!("Value a from function in extra thread: {}", b);
}
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {

    // create send/receiver vars
    // to move data through channel
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {

        let num1 = 1;
        let num2 = 2;
        let num3 = 3;

        tx.send(num1).unwrap();
        tx.send(num2).unwrap();
        tx.send(num3).unwrap();
        // wait .5 sec between sendings
        thread::sleep(Duration::from_millis(500));
    });

    handle.join().unwrap();

    // access rx directly
    // and use values
    for i in rx {

        println!("{}", i);
        thread::sleep(Duration::from_millis(500));
    }
}
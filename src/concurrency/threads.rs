use std::thread;
use std::time::Duration;

pub fn run() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Loop 2 iteration: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
        22 // Return 22 from the thread
    });

    for i in 0..5 {
        println!("Loop 1 iteration: {}", i);
        thread::sleep(Duration::from_millis(500));
    }

    let res = handle.join().unwrap();
    println!("{:?}", res); // This should now print 22
}
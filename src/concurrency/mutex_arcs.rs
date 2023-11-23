/*
A data race is caused when more than one thread is trying to mutate data at the same time.

The compiler doesn’t know which one to do first, so it panics. Rust provides us with Mutex and Arc to handle data races.
Mutex is a container that holds and locks the data it’s currently working on, to prevent anything else from trying to mutate it.

It implements these locks in a data-driven way. We have to specify the type of data protected by the mutex, and Rust will ensure the data is only accessed through the mutex. In other words, “lock data, not code”.

Arc
Arc provides thread-safe shared ownership of a value. Simply put, we use Arc to share memory between threads.
Arc ensures that the data it points to is not destroyed until all references to it have been dropped. Each clone of an Arc increases the reference count, and the data is only cleaned up when the last Arc pointing to it is dropped.

Remember that when working with Arc, we’re working with memory, so when the value is mutated or accessed, we have to dereference it with the * operator.


When sharing data across multiple threads without using Arc, if one thread finishes its execution and drops the data (because it owns the data), the following issues can arise:
Problems without Arc (Atomic Ref Counting):-
- Premature Deallocation: The data is deallocated once the owning thread finishes and drops the data. If other threads are still running and expecting to use this data, they will encounter an error because the data they are trying to access no longer exists.
- Dangling References: Other threads might end up with dangling references (pointers to memory that has been freed). Accessing such memory leads to undefined behavior, which is a severe error in Rust and can lead to program crashes, data corruption, or security vulnerabilities.
- Data Races and Concurrency Errors: Without proper synchronization and shared ownership (like that provided by Arc), you also risk data races and other concurrency-related errors. These issues are notoriously difficult to debug and can lead to erratic program behavior.

Solution with Arc
Arc solves these problems by ensuring that:
- Shared Ownership: Multiple threads can own a shared piece of data. The data will not be deallocated until all owners have finished using it (i.e., all Arc instances pointing to the data are dropped).
- Thread-Safe Reference Counting: Arc uses atomic operations to manage the reference count, ensuring that it's safe to increment and decrement this count across different threads.
- Automatic Memory Management: The data is automatically deallocated when the last Arc pointing to it is dropped. This prevents premature deallocation and ensures that all threads have finished using the data before it's freed.
 */

use std::thread;
use std::sync::{Mutex, Arc};

pub fn run() {

    // thread-safe and lockable
    let safe = Arc::new(Mutex::new(5));

    // more than one handle
    // store them in a vec
    // for convenience
    let mut handles = vec![];

    for i in 0..2 {
        // creating new instance of arc and increment ref count
        let safe = Arc::clone(&safe);
        // create the thread
        let handle = thread::spawn(move|| {

            // lock the value (only one thread can mutate it)
            let mut a = safe.lock().unwrap();
            // mutate the value
            *a += 3;
        });

        // push the handle into the handles
        // vector so we can join them
        handles.push(handle);
    }

    // join the handles in the vector
    for i in handles {
        i.join().unwrap();
    }

    // lock the value when accessing it
    println!("a: {}", *safe.lock().unwrap());
}
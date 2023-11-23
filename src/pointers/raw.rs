/*


Raw Pointers
Types: Immutable (*const T) and Mutable (*mut T).
Safety: Considered unsafe; they can be null, dangling, or point to invalid memory.
Explicit Dereferencing: Must be explicitly dereferenced using the * operator within an unsafe block.
Usage:
Often used for low-level memory manipulation, interfacing with C code, or when building safe abstractions manually.
Requires careful handling due to the lack of safety guarantees.
Function Parameters: Can be passed to functions, but operations on them must be enclosed in unsafe blocks. Used in scenarios where Rust's safety guarantees need to be bypassed for greater control.
 */
fn print_raw_value(val: *const i32) {
    unsafe {
        println!("Value: {}", *val);
    }
}

fn modify_raw_value(val: *mut i32) {
    unsafe {
        *val += 1;
    }
}

pub fn run() {
    let x = 10;
    let raw_ptr = &x as *const i32;
    unsafe {
        // modify_raw_value(raw_ptr);
        print_raw_value(raw_ptr);
    }
}


/*
A slice is a pointer to a block of memory. Slices can be used to access portions of data stored in contiguous memory blocks. It can be used with data structures like arrays, vectors and strings. Slices use index numbers to access portions of data. The size of a slice is determined at runtime.

Slices are pointers to the actual data. They are passed by reference to functions, which is also known as borrowing.
 */

pub fn run(){
    let data = [10,20,30,40,50];
    use_slice(&data[1..4]);
    //this is effectively borrowing elements for a while
}
fn use_slice(slice:&[i32]) {
    // is taking a slice or borrowing a part of an array of i32s
    println!("length of slice is {:?}",slice.len());
    println!("{:?}",slice);
}
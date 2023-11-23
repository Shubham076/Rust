/*
A Vector is a resizable array. It stores values in contiguous memory blocks. The predefined structure Vec can be used to create vectors.
let mut instance_name = Vec::new();
let vector_name = vec![val1,val2,val3]
 */

pub fn run() {
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);

    println!("size of vector is :{}",v.len());
    println!("{:?}",v);

    let vt = vec![1,2,3];
    println!("{:?}",vt);

    v.remove(1);

    if v.contains(&10) {
        println!("found 10");
    }

    println!("{:?}",v[0]);

    for i in &v {
        println!("{}",i);
    }
}
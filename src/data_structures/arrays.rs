/*
An array is a collection of objects of the same type T, stored in contiguous memory. Arrays are created using brackets [], and their length, which is known at compile time
 primitives

 //Syntax1
let variable_name = [value1,value2,value3];

//Syntax2
let variable_name:[dataType;size] = [value1,value2,value3];

//Syntax3
let variable_name:[dataType;size] = [default_value_for_elements,size];
let arr:[i32;4] = [-1;4]
output = [-1, -1, -1, -1]
 */

fn analyze_array(array: [i32;4]) {
    println!("First element of the slice: {}", array[0]);
    println!("The slice has {} elements", array.len());
}

pub fn run() {
    let arr:[i32;4] = [10,20,30,40];
    println!("array is {:?}",arr);
    println!("array size is :{}",arr.len());

    println!("Number of elements in array: {}", arr.len());

    for index in 0..4 {
        println!("index is: {} & value is : {}",index,arr[index]);
    }

    for val in arr.iter(){
        println!("value is :{}",val);
    }

    analyze_array(arr)
}


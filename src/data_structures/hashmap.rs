/*

let mut instance_name = HashMap::new();
 */
use std::collections::HashMap;

pub fn main() {
    let mut state_codes = HashMap::new();
    state_codes.insert("KL", "Kerala");
    state_codes.insert("MH", "Maharashtra");
    println!("{:?}", state_codes);

    println!("size of map is {}", state_codes.len());

    match state_codes.get(&"KL") {
        Some(value)=> {
            println!("Value for key KL is {}",value);
        }
        None => {
            println!("nothing found");
        }
    }

    for (key, val) in state_codes.iter() {
        println!("key: {} val: {}", key, val);
    }

    if state_codes.contains_key(&"GJ") {
        println!("found key");
    }

    state_codes.remove(&"GJ");
}
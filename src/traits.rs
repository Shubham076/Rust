/*

A trait tells the Rust compiler about functionality a particular type has and can share with other types. Traits are an abstract definition of shared behavior amongst different types.
 */

trait MyTrait {
    fn do_something(&self);
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}


struct MyStruct;

impl MyTrait for MyStruct {
    fn do_something(&self) {
        println!("Doing something!");
    }
}

//one way
// fn my_function<T: MyTrait>(item: T) {
//     item.do_something();
// }

fn my_function(item: impl MyTrait) {
    item.do_something();
}

pub fn run() {
    let my_struct = MyStruct;
    my_function(my_struct);
}

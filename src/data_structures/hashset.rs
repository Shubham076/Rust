use std::collections::HashSet;
pub fn run() {
    let mut names = HashSet::new();

    names.insert("Mohtashim");
    names.insert("Kannan");
    names.insert("TutorialsPoint");
    names.insert("Mohtashim");//duplicates not added

    println!("{:?}",names);
    println!("size of the set is {}",names.len());

    for name in names.iter() {
        println!("{}",name);
    }

    match names.get(&"Mohtashim"){
        Some(value)=>{
            println!("found {}",value);
        }
        None =>{
            println!("not found");
        }
    }


    if names.contains(&"Kannan") {
        println!("found name");
    }
}
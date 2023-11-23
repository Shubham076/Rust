pub fn run() {

    // loop keyword
    let mut a = 5;
    loop {
        if a == 10 {
            break
        }
        println!("{a}");
        a += 1;
    }

    //while
    a = 0;
    while a < 5 {
        println!("{a}");
        a += 1;
    }

    //for 
    for x in 1..11{ // 11 is not inclusive
        if x==5 {
            continue;
        }
        println!("x is {}",x);
    }
}
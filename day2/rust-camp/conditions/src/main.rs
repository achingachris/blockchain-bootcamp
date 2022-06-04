use std::cmp::Ordering; 


fn main() {
    println!("Program Starting");

    let height: i32  = 39;
    if height > 50 {
        println!("You are tall");
    } else {
        println!("You are average");
    }

    match height.cmp(&50) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win?"),
    }

    match height {
        10 => println!("You are 10 feet tall"),
        10..=40 => println!("You are between 10 and 40 feet tall"),
        _ => println!("unkown"),
    }

    println!("Program End");

}

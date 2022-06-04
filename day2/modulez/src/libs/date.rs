use chrono::prelude::*;

pub fn date() {
    // get current date
    let now = Local::now();
    println!("today");
    println!("{}", now.format("%Y-%m-%d"));

}


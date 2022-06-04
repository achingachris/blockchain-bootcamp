mod username;
mod libs;

use username::username;
use libs::date::date;
use libs::time::time;

fn main() {
    println!("Hello, world!");
    username();
    date();
    time();
}

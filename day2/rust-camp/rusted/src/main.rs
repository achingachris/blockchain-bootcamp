// #[allow(dead_code)]


fn main() {
    let user_name = String::from("Chris");
    let height :u8 = 30;
    println!("Hello, world!");

    print_user();
    data_types_playground();
    sample();
    sample_one(user_name, height);
}

// simple print 
fn print_user() {
    println!("Hello, Chris!");
}

#[allow(unused_variables)]
fn data_types_playground() {
    // booleans
    let  is_active :bool = true;

    // unsigned 
    let int_1 :u8 = 255;
    let int_2 :u16 = 256;
    let int_3 :u32 = 257;
    let int_4 :u64 = 258;
    let int_5 :u128 = 259;
    let int_6 :usize = 260;

    // signed
    let int_7 :i8 = -1;
    let int_8 :i16 = -2;
    let int_9 :i32 = -3;
    let int_10 :i64 = -4;
    let int_11 :i128 = -5;
    let int_12 :isize = -6;

    // floats
    let float_1 :f32 = 1.0;
    let float_2 :f64 = 2.0;

    // str
    let name_str :&str = "Chris";

    // String
    let mut name :String = String::from("Chris Achinga");
    // name.push_str(" is a Rust developer");
    name.push_str(&" ch".to_string());

    // print name
    println!("{}", name);

}

// sample functions
#[allow(dead_code)]
fn sample() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn sample_one(user_name:String, height:u8) {
    println!("Running Sample One");
    println!("Hello, {}! Your height is {}", user_name, height);
}

#[allow(dead_code)]
fn sample_two() -> u8 {
    println!("Running Sample Two");
    return 10;
}

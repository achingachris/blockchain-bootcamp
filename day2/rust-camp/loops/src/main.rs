fn main() {
    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    for elem in numbers.iter() {
        println!("{}", elem);
    }
    
    

    println!("\n");
    println!("===============================================================");

    let mut index: usize = 0;
    while index != numbers.len() {
        println!("{}", numbers[index]);
        index += 1;
    }
}

struct Rectange {
    height: i32,
    width: i32,
}

impl Rectange {
    fn preimeter(&self) -> i32 {
        (self.height * 2) + (self.width * 2)
    }

    fn area(&self) -> i32 {
        (self.height) * (self.width)
    }
}

fn main() {
    let my_rectangle = Rectange {
        height: 10,
        width: 20,
    };

    println!("The perimeter is {}", my_rectangle.preimeter());
    println!("The area is {}", my_rectangle.area());
}

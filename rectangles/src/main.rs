#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        return self.width > other_rect.width && self.height > other_rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 40, height: 60 };
    let rect3 = Rectangle { width: 50, height: 70 };
    let square1 = Rectangle::square(80);

    println!("The area of rectangle 1 is {} square pixels", rect1.area());
    //println!("info about rectangle 1: {:#?}", rect1); 
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    println!("square1: {:#?}", square1);
}

/*
fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/
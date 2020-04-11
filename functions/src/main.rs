fn main() {
    println!("Hello, world!");
    another_function(64, 32);

    let x = 15;

    let y = {
        let x = 4;
        x + 1
    };

    println!("the values of x and y are {}, {}", x, y);
    println!("gimme five {}", five());
    let x = 10;
    println!("adding 1 to {} yields {}", x, plus_one(x));
}

fn another_function(x: i32, y: i32) {
    println!("Hello from another function where the value of x is {} and y is {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    /*
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x); 
    */
    /*
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x is now {}", x);
    */
    /*
    let x = 2.1;
    let y: f32 = 3.0;

    println!("{} + {} = {}", x, y, x + y);
    */
    /*
    let tup: (i32, char, bool) = (34, 'h', true);
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("x = {}, y = {}, z = {}", tup.0, tup.1, tup.2);
    */
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("This will fail {}", array[100]);
}
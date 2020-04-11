fn main() {
    /*
    let number = 15;
    if number < 10 {
        println!("true");
    } else {
        println!("false");
    }
    */
    /*
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("number is: {}", number);
    */
    /*
    loop {
        println!("weeee, again");
    }
    */
    /*
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is: {}", result)
    */

    /*
    let mut counter = 3;
    while counter > 0 {
        println!("{}", counter);
        counter -= 1;
    }
    println!("LIFTOFF!!");
    */
    let items = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for item in items.iter() {
        println!("{}", item);
    }

    for number in (1..4).rev() {
        println!("{}", number); 
    }
    println!("LIFTOFF");
}

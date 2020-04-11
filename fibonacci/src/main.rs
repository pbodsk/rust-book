use std::io;

fn main() {
    println!("How may iterations?");

    let mut iterations = String::new();
    io::stdin().read_line(&mut iterations)
        .expect("Failed to read line");

    let iterations: u32 = iterations.trim().parse()
        .expect("Wrong number");


    let mut values = vec![];
    let mut cur_pos = 0;
    while cur_pos < iterations {
        if values.is_empty() {
            values.push(0);
            values.push(1);
            cur_pos += 2;
        }
        let last_item = values[values.len() - 1];
        let second_to_last_item = values[values.len() -  2]; 
        let result = add(second_to_last_item, last_item);
        values.push(result);
        cur_pos += 1;
    }
    for val in values.iter() {
        print!("{}, ", val);
    }
    println!("");
}

fn add(first: i32, second: i32) -> i32 {
    first + second
}

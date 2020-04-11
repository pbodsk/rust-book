use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut number_of_guesses = 0;
    println!("The - not so - secret number is: {}", secret_number);
    println!("Guess the number");
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("Your guess: {}", guess);
    
        number_of_guesses += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Equal => {
                println!("Thats right! You used {} guesses to figure this out", number_of_guesses);
                break;
            }
            Ordering::Greater => println!("Too high")
        }    
    }
}

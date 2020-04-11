fn main() {
    /*
    let mut s = String::from("Hello");
    s.push_str(", world");
    println!("{}", s);
    let mut s2 = s.clone();
    println!("{} - {}", s, s2); 
    s2.push_str(" with more data");
    println!("{} - {}", s, s2); 
    */

    let some_string = String::from("What value will I have?");
    takes_ownership(some_string);
    //println!("accessed? {}", some_string);

    let some_int = 42;
    makes_copy(some_int);
    println!("accessed? {}", some_int);

    let from_func = gives_ownership();
    println!("{}", from_func);

    let input_string = String::from("To: Function\nFrom: Peter");
    let output_string = takes_and_gives_back(input_string);
    println!("{}", output_string); 

    let my_name = String::from("Peter");
    let length = calculate_length(&my_name);
    println!("The length of {} is {}", my_name, length);

    change(&mut String::from("There we go"));

    //dangle();

    let full_name = String::from("Peter Bødskov");
    let slice = first_word(&full_name);
    println!("{}", slice);

    let freja = first_word("Freja Davidsen Bødskov");
    println!("{}", freja); 

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    String::from("I got this from a function, thanks!")
}

fn takes_and_gives_back(a_string: String) -> String {
    let mut updated = a_string.clone();
    updated.push_str("\n Why thanks");
    updated
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" break stuff!");
    println!("{}", s);
}

/*
fn dangle() -> &String {
    let s = String::from("Hello");
    &s
}
*/

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
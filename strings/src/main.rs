fn main() {
    let empty_string = String::new();

    let data = "initial contents";
    let s = data.to_string();

    //directly
    let s2 = "initial contents".to_string();

    //or
    let s3 = String::from("initial contents");
    
    let mut name = String::from("Peter");
    name.push_str(" Bødskov");

    println!("{}", name);

    let hello = String::from("Hello");
    let world = String::from("World");
    let hello_world = hello + " " +  &world;
    println!("{}", hello_world );

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let tic_tac_toe = format!("{}-{}-{}", tic, tac, toe);
    println!("{}", tic_tac_toe);

    let hello = "Здравствуйте";
    //Will crash as we're cutting the string inside a unicode char (not on a char boundary)
    //let start = &hello[0..1];
    //println!("{}", start);
}

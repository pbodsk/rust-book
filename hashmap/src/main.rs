fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 40);

    /*
    let tup1: (String, i32) = (String::from("Peter"), 100);
    let tup2: (String, i32) = (String::from("Pia"), 50);
    let tuples = [tup1, tup2];
    */

    let names = [String::from("Peter").to_lowercase(), String::from("Pia").to_lowercase()];
    let scores = [10, 50];

    let scores2: HashMap<_,_> = names.iter().zip(scores.iter()).collect();
    println!("{:#?}", scores2);

    //Ownership
    let field_name = String::from("favorite color");
    let field_value = String::from("black");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    

    //Fails as the map owns field_name and field_value once inserted
    //println!("{:#?} {} {}", map, field_name, field_value);

    //And this won't work since we borrowed the value as immutable
    //map.insert(&field_name, &field_value);
    //field_value = String::from("red");
    //println!("{:#?} {} {}", map, field_name, field_value);

    let key = String::from("favorite color");
    let val = map.get(&key);
    println!("{:?}", val);

    for (key, value) in scores2 {
        println!("k: {} - v: {}", key, value);
    }

    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);
    scores3.insert(String::from("Yellow"), 50);

    println!("{:?}", scores3);

    //insert if not exists
    scores3.entry(String::from("Blue")).or_insert(80);
    scores3.entry(String::from("Red")).or_insert(80);

    println!("{:?}", scores3);

    println!("count occurences of word");

    let mut word_count = HashMap::new();
    let text = "Hello world how are you world all good world";
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_count);

}

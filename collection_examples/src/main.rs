use std::collections::HashMap;

fn main() {
    //vector_examples();
    pig_latinize();
}

fn vector_examples() {
    println!("***** Vector examples start *****");

    let ints = vec![10, 30, 20, 50, 40, 80, 60, 90, 100, 70, 110, 20];

    let mut total = 0;
    for val in &ints {
        total += val;
    }
    let avg = total / ints.len();
    println!("Average: {}", avg);

    let mut sorted_ints = ints.clone();
    sorted_ints.sort();

    let middle_pos = sorted_ints.len() / 2;
    let median = sorted_ints.get(middle_pos);
    println!("Median: {:?}", median);
    println!("{:?}", sorted_ints);

    let mut item_count = HashMap::new();

    for val in &ints {
        let count = item_count.entry(val).or_insert(0);
        *count += 1;
    }

    println!("occurences {:?}", item_count);

    println!("***** Vector examples end *****");
}

fn pig_latinize() {
    let vocals = vec![
        String::from("a"),
        String::from("e"),
        String::from("i"),
        String::from("o"),
        String::from("u"),
    ];

    println!("***** pig latin examples start *****");
    let input_string = "The quick brown fox jumps over the lazy dog";
    let mut new_sentence: Vec<String> = Vec::new();
    for word in input_string.split_whitespace() {
        let first_letter = word.to_string().chars().next().unwrap().to_lowercase().to_string();
        let last_part = &word[1..];
        
        if vocals.contains(&first_letter) {
            let mut m = String::new();
            m.push_str(word);
            m.push_str("-hay");
            new_sentence.push(m);
        } else {
            let mut m = String::new();
            m.push_str(last_part);
            m.push_str("-");
            m.push_str(&first_letter);
            m.push_str("ay");
            new_sentence.push(m);
        }
    }
    for new_word in new_sentence {
        print!("{:?} ", new_word);
    }
    //println!("{:?}", new_sentence);
    println!("***** pig latin examples end *****");
}

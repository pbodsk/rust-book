fn main() {

    //Vectors (arrays)
    let v: Vec<i32> = Vec::new();

    //or
    let i_vec = vec![1, 2, 3];


    let mut mutable_vector = vec![1, 2, 3];
    mutable_vector.push(4);
    mutable_vector.push(5); 
    mutable_vector.push(6);
    mutable_vector.push(7);
    let pos = 2;

    let third = &mutable_vector[pos];
    
    
    //mutable_vector.push(8);
    //will crash if the above line is added, 
    //since pushing an element might cause the vector to change position
    //in memory, meaning that `third` would end up pointing to invalid
    //data
    println!("third element is {}", third);

    match mutable_vector.get(pos) {
        Some(elem) => println!("found {}", elem),
        None => println!("There is no element at pos {}", pos)
    }

    for val in &mutable_vector {
        println!("{}", val);
    }

    mutable_vector.push(8);

    //change values using dereference operator *
    for val in &mut mutable_vector {
        *val += 100;
    }

    for val in &mutable_vector {
        println!("{}", val);
    }
}
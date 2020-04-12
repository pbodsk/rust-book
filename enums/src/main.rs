enum IPAddrKind {
    v4,
    v6
}

enum IPAddr {
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("A Quarter from {:?}", state);
            25
        },
    }
}

fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    let value = value_in_cents(&coin);
    println!("A {:?} ia worth {} cents", coin, value);

    println!("{:?}", add_one(Some(5)));
    println!("{:?}", add_one(None));

    let optional_int = Some(4);
    if let Some(4) = optional_int {
        println!("Yo!")
    } else {
        println!("No!") 
    }
}


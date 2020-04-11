struct User {
    username: String,
    email: String,
    sign_in_count: i64,
    active: bool
}

fn main() {
    let mut user = build_user(String::from("pebo@nodes.dk"), String::from("pbodsk"));
    println!("{}", user.username);
    user.username = String::from("pebo");
    println!("{}", user.username);

    let mut user2 = User {
        email: String::from("pia@piskeris.dk"),
        username: String::from("pida"),
        ..user
    };
    println!("before - user {}", user.active);
    println!("before - user2 {}", user2.active);
    user2.active = false;
    println!("after - user {}", user.active); 
    println!("after - user2 {}", user2.active);


}

fn build_user(email: String, username: String) -> User {
    let user = User {
        email,
        username,
        sign_in_count: 1,
        active: true
    };
    user
}

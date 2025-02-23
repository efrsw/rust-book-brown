struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 42
    }
}

fn main() {
    let user1 = build_user(String::from("Username"), String::from("User email"));
    println!("{}", user1.username);

    let mut user2 = User {
        active: false,
        ..user1
    }
    user2.email.push_str("Another email");
}

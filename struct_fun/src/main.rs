struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

fn main() {
    let user1 = build_user(String::from("randychuj@chuj.welltok"),
                           String::from("randychuj"));

    println!("{}", user1.email);

    let black = Color(0, 0, 0);
    println!("{} {} {}", black.0, black.1, black.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
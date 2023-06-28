struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("name : {}", user1.username);
    println!("email : {}", user1.email);
    println!("active : {}", user1.active);
    println!("sign_in_count : {}", user1.sign_in_count);
}

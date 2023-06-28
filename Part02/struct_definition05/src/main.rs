struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = build_user(String::from("suho.kwon@gmail.com"), String::from("Suho Kwon"));

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    println!("name : {}", user2.username);
    println!("email : {}", user2.email);
    println!("active : {}", user2.active);
    println!("sign_in_count : {}", user2.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

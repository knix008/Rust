struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = build_user(String::from("suho.kwon@gmail.com"), String::from("Suho Kwon"));

    println!("name : {}", user.username);
    println!("email : {}", user.email);
    println!("active : {}", user.active);
    println!("sign_in_count : {}", user.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

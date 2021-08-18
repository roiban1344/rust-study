struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count:1 ,
    };

    user1.email = String::from("anotheremail@example.com");
    user1.sign_in_count += 1;
    let User { email, username, active, sign_in_count } = user1;
    println!("{} {} {} {}", email, username, active, sign_in_count);

    let user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("{} {}", user2.email, user2.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(f64, f64, f64);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("myuser1"),
        email: String::from("myemail1@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    println!("{}", user1.email);

    let user2 = build_user(
        String::from("myUser2"),
        String::from("myemail2@example.com"),
    );
    println!("{}", user2.email);

    let user3 = User {
        active: user2.active,
        username: user1.username,
        email: String::from("anotherone@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("{}", user3.email);

    let black = Color(0, 0, 0);
    let point = Point(0.0, 0.0, 0.0);

    println!("{}", black.1);

    let subject = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

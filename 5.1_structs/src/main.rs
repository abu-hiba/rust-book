fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("username"),
        email: String::from("username@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);

    let user2 = build_user(String::from("user2"), String::from("user2@email.com"));
    println!("{}", user2.active);
    println!("{}", user2.username);
    println!("{}", user2.email);
    println!("{}", user2.sign_in_count);

    let user3 = User {
        email: String::from("user3@email.com"),
        ..user2
    };

    println!("{}", user3.email);

    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}, {}", black.0, origin.2);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

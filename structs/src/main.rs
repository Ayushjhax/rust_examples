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

    let user2 = User {
        email: String::from("another@example.com"),
        username: user1.username,
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("user1: {user1.username}");
    println!("user2: {user2.username}");
    println!("user3: {user3.username}");

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));
}
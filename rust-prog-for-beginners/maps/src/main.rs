#[derive(Debug)]
struct User {
    id: i32,
    name: String,
}

fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "dave" => Some(9),
        _ => None,
    }
}

fn main() {
    let name: &str = "tim";
    let user = find_user(name).map(|found| User {
        id: found,
        name: name.to_string(),
    });
    match user {
        Some(u) => println!("{:?}", u),
        None => println!("User not found"),
    }
}

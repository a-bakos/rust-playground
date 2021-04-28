fn main() {
    let maybe_user = Some("Tom");
    match maybe_user {
        Some(user) => println!("user={:?}", user),
        None => println!("No user"),
    }

    // if there was a situation where you only cared
    // if there was a user and nothing happens if
    // there was nothing, can use if-let.
    // This if-let is functionally equivalent to
    // matching only the first arm above in match
    if let Some(user) = maybe_user {
        println!("user={:?}", user);
    } else {
        println!("no user");
    }

    // IF-LET on enums
    let color = Color::Red;
    if let Color::Red = color {
        println!("This is red");
    } else {
        println!("Not red");
    }
}

enum Color {
    Red,
    Green,
    Black,
}

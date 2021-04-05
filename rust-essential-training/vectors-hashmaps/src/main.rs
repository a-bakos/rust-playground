fn main() {
    let mut friends: Vec<String> = Vec::new();
    friends.push(String::from("Joey"));
    friends.push(String::from("Ross"));
    friends.push(String::from("Chandler"));

    println!("Friends are: {:?}", friends);

    let third = &friends[2];
    println!("The third friend is {}", third);
}

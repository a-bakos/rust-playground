fn main() {
    let mut friends: Vec<String> = Vec::new();
    friends.push(String::from("Joey"));
    friends.push(String::from("Ross"));
    friends.push(String::from("Chandler"));
    friends.push(String::from("Gunther"));

    println!("Friends are: {:?}", friends);

    let last = friends.pop();
    println!("Last friend is {:?}", last);
    //let fourth = &friends[3];
    //println!("Try to get Gunther {}", fourth); // out of bounds error
    let fourth = friends.get(3);
    println!("Try to get Gunther: {:?}", fourth); // None

    println!("Friends are: {:?}", friends);

    let third = &friends[2];
    println!("The third friend is {}", third);

    // Pre-populate the vector with values:
    let countdown = vec![5, 4, 3, 2, 1];
}

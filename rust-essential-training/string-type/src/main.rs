fn main() {
    let mut message = String::from("Earth"); // Allocated on the heap
    println!("{}", message);
    message.push_str(" is home");
    println!("{}", message);
}

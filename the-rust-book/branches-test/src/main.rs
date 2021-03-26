fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, branches test");

    let result = plus_one(3);

    if result < 5 {
        println!("Less than five; true");
    } else {
        println!("More than or equal to five; false");
    }

    let condition = true;

    let number = if condition { 10 } else { 100 };

    println!("Using 'if' in a let statement. Equals to: {}", number);
}

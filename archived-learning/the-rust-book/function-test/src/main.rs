fn get_five(x: i8) -> i8 {
    x // no semicolon here
}

fn main() {
    println!("Hello, function test!");

    let is_it_five = get_five(5);

    println!("The value of the variable is {}", is_it_five);
}

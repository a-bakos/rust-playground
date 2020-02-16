fn get_five() -> i8 {
    5
}

fn main() {
    println!("Hello, function test!");

    let is_it_five = get_five();

    println!("The value of the variable is {}", is_it_five);
}

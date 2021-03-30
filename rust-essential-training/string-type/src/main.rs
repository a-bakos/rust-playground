fn main() {
    let mut message = String::from("Earth"); // Allocated on the heap
    println!("{}", message);
    message.push_str(" is home");
    println!("{}", message);

    // String ownership
    let rocket_fuel = String::from("RP-1");
    // Shadow the prev variable
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("Rocket_fuel is {}", rocket_fuel);

    fn process_fuel(mut propellant: String) -> String {
        println!("Processing propellant {}", propellant);
        propellant.push_str(" V2");
        // we're transferring ownership back
        propellant
    }
}

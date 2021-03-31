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

    let fuel_len = borrow_fuel(&rocket_fuel);
    println!("Borrowed fuel length is {}", fuel_len);

    fn process_fuel(mut propellant: String) -> String {
        println!("Processing propellant {}", propellant);
        propellant.push_str(" V2");
        // we're transferring ownership back
        propellant
    }

    // Borrowing
    fn borrow_fuel(propellant: &String) -> usize {
        println!("Borrowing propellant");
        let length = propellant.len();
        length
    }

    // Mutable borrowing
    let mut rocket_fuel_2 = String::from("RP-2");
    fn mutable_borrow_fuel(propellant: &mut String) -> usize {
        propellant.push_str(" + Added bit here");
        let length = propellant.len();
        length
    }
    mutable_borrow_fuel(&mut rocket_fuel_2);
    println!("After processing rocket fuel 2: {}", rocket_fuel_2);

    /**
     * Slices
     */
    let new_message = String::from("Greetings from Earth!");
    let earth_slice = &new_message[15..=15 + 5];
    println!("The slice is {}", earth_slice);
}

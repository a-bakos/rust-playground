use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        let state: String = state.trim().to_lowercase();
        // String -> &str
        match state.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }
}

fn print_power_state(state: PowerState) {
    use PowerState::*; // shorthand for PowerState
    match state {
        Off => println!("Turning off"),
        Sleep => println!("Going to sleep"),
        Reboot => println!("Rebooting"),
        Shutdown => println!("Shutting down"),
        Hibernate => println!("Hibernating"),
    }
}

fn main() {
    let mut user_input_buffer: String = String::new();
    println!("Enter power state:");
    let user_input_status = io::stdin().read_line(&mut user_input_buffer);

    // is_ok is defined on Results
    if user_input_status.is_ok() {
        // convert the String to a string slice by referencing
        match PowerState::new(&user_input_buffer) {
            Some(state) => print_power_state(state),
            None => println!("Invalid power state"),
        }
    } else {
        println!("Error reading input");
    }
}

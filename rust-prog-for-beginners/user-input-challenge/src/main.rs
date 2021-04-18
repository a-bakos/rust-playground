use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
    Error,
}

fn print_power_state(state: PowerState) {
    match state {
        PowerState::Off => println!("Turning off"),
        PowerState::Sleep => println!("Going to sleep"),
        PowerState::Reboot => println!("Rebooting"),
        PowerState::Shutdown => println!("Shutting down"),
        PowerState::Hibernate => println!("Hibernating"),
        PowerState::Error => println!("Error!"),
    }
}

fn main() {
    let mut user_input_buffer: String = String::new();
    io::stdin().read_line(&mut user_input_buffer);
    let the_input = user_input_buffer.trim().to_lowercase();
    let the_state = match the_input.as_str() {
        "off" => PowerState::Off,
        "sleep" => PowerState::Sleep,
        "reboot" => PowerState::Reboot,
        "shutdown" => PowerState::Shutdown,
        "hibernate" => PowerState::Hibernate,
        _ => PowerState::Error,
    };
    print_power_state(the_state);
}

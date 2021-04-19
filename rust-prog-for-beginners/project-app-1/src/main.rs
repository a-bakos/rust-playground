use std::io;

const MENU_EXIT: &str = "exit";

fn main() {
    loop {
        let mut buffer: String = String::new();
        let user_input = io::stdin().read_line(&mut buffer);

        if user_input.is_ok() {
            break;
        }
    }
}

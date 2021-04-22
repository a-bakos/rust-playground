use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Bill {
    name: String,
    balance: f64,
}
impl Bill {
    fn new(name: String, bal: f64) -> Self {
        Bill {
            name: name,
            balance: bal,
        }
    }
}

#[derive(Debug)]
struct Bills {
    inner: Vec<Bill>,
}
impl Bills {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.push(bill);
    }

    fn get_all(&self) -> &Vec<Bill> {
        &self.inner
    }
}

#[derive(Debug)]
enum Menu {
    Add,
    Remove,
    View,
    Edit,
    Back,
    Exit,
}

impl Menu {
    fn new(choice: &str) -> Option<Menu> {
        let choice: String = choice.trim().to_lowercase();

        match choice.as_str() {
            "add" => Some(Menu::Add),
            "remove" => Some(Menu::Remove),
            "view" => Some(Menu::View),
            "edit" => Some(Menu::Edit),
            "back" => Some(Menu::Back),
            "exit" => Some(Menu::Exit),
            _ => None,
        }
    }
}

fn get_input() -> String {
    let mut buffer: String = String::new();
    let user_input_status = io::stdin().read_line(&mut buffer); // returns Result type: Ok/Err

    if user_input_status.is_ok() {
        println!("{:?}", buffer);
        buffer.trim().to_owned()
    } else {
        "Invalid menu choice".to_owned()
    }
}

fn main() {
    loop {
        println!("What would you like to do?");
        let menu = get_input();

        if menu == "exit" {
            break;
        }
    }
}

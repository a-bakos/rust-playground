use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Bill {
    name: String,
    balance: f64,
}

#[derive(Debug)]
struct Bills {
    bills: Vec<Bill>,
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

fn get_bill(bill_name: String, bills: HashMap<String, i32>) -> i32 {
    let bill_val = bills.get(&bill_name);
    bill_val
}

fn menu_placeholder() {
    println!("Placeholder");
}

fn main() {
    let mut my_bills: HashMap<String, i32> = HashMap::new();
    my_bills.insert(String::from("testbill"), 250);

    loop {
        let mut buffer: String = String::new();
        println!("What would you like to do?");
        let user_input_status = io::stdin().read_line(&mut buffer); // returns Result type: Ok/Err

        if user_input_status.is_ok() {
            let menu_selected: Option<Menu> = Menu::new(&buffer);
            match menu_selected {
                Some(menu_choice) => {
                    println!("{:?}", menu_choice);
                    if menu_choice == menu_selected {
                        println!("yas");
                    }
                }
                None => println!("Invalid menu"),
            }
        } else {
            println!("Error reading input!");
            break;
        }
    }
}

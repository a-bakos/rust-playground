use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
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
    inner: HashMap<String, Bill>,
}
impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.clone(), bill);
    }

    fn get_all(&self) -> Vec<Bill> {
        let mut bills = vec![];
        for bill in self.inner.values() {
            bills.push(bill.clone());
        }
        bills
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
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
        println!("{}", buffer);
        buffer.trim().to_owned()
    } else {
        "Invalid menu choice".to_owned()
    }
}

fn add_bill_menu(bills: &mut Bills) {
    let name: String = get_input();
    let amount: f64 = get_bill_amount();
    let current_bill = Bill::new(name, amount);
    bills.add(current_bill);
    println!("Bill successfully added");
}

fn remove_bill_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    let input = get_input();
    if bills.remove(&input) {
        println!("Removed");
    } else {
        println!("Bill not found");
    }
}

fn view_bills_menu(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
}

fn get_bill_amount() -> f64 {
    println!("Enter amount:");
    loop {
        let amount: String = get_input();
        let parsed: Result<f64, _> = amount.parse();
        match parsed {
            Ok(num) => return num,
            Err(_) => println!("Enter a valid number!"),
        }
    }
}

fn main_menu() {
    let mut bills = Bills::new();

    loop {
        show_menu();
        let input = get_input();
        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            "3" => remove_bill_menu(&mut bills),
            _ => break,
        }
    }
}
fn show_menu() {
    println!("");
    println!("__ Manage Bills App __");
    println!("");
    println!("1 - Add Bill");
    println!("2 - View Bills");
    println!("3 - Remove Bill");
    println!("");
    println!("Enter menu:");
}

fn main() {
    main_menu();
}

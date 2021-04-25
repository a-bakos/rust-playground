use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    balance: f64,
}
impl Bill {
    fn new(name: String, bal: f64) -> Self {
        Bill { name, balance: bal }
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

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.balance = amount;
                true
            }
            None => false,
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer: String = String::new();

    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}

fn add_bill_menu(bills: &mut Bills) {
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };
    let amount: f64 = match get_bill_amount() {
        Some(input) => input,
        None => return,
    };
    let current_bill = Bill::new(name, amount);
    bills.add(current_bill);
    println!("Bill successfully added");
}

fn remove_bill_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    let input = match get_input() {
        Some(input) => input,
        None => return,
    };
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

fn edit_bill_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };
    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };

    if bills.update(&name, amount) {
        println!("Updated");
    } else {
        println!("Bill not found");
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Enter amount:");
    loop {
        let amount = match get_input() {
            Some(amount) => amount,
            None => return None,
        };
        if amount.is_empty() {
            return None;
        }
        let parsed: Result<f64, _> = amount.parse();
        match parsed {
            Ok(num) => return Some(num),
            Err(_) => println!("Enter a valid number!"),
        }
    }
}

fn main_menu() {
    let mut bills = Bills::new();

    loop {
        show_menu();
        let input = match get_input() {
            Some(input) => input,
            None => return,
        };
        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            "3" => remove_bill_menu(&mut bills),
            "4" => edit_bill_menu(&mut bills),
            _ => break,
        }
    }
}
fn show_menu() {
    println!("__ Manage Bills App __");
    println!("1 - Add Bill");
    println!("2 - View Bills");
    println!("3 - Remove Bill");
    println!("4 - Edit Bill");
    println!("Enter menu:");
}

fn main() {
    main_menu();
}

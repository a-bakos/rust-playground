#[derive(Debug)]
enum Menu {
    Main,
    Start,
    Quit,
}

fn get_choice(choice: &str) -> Result<Menu, String> {
    match choice {
        "main" => Ok(Menu::Main),
        "start" => Ok(Menu::Start),
        "quit" => Ok(Menu::Quit),
        _ => Err("Menu choice not found".to_owned()),
    }
}

fn print_choice(choice: &Menu) {
    println!("choice: {:?}", choice);
}

// If you want to return nothing from a function,
// you can use the unit type: ()
fn pick_choice(choice: &str) -> Result<(), String> {
    // the ? mark will automatically perform a match operation:
    // if the return value is Ok(), that'll be the variable's value,
    // if it's Err(), it'll return from the wrapping function (Hint: String)
    let choice: Menu = get_choice(choice)?;
    // If the question mark above evaulates to Err(),
    // the below code will never be executed:
    print_choice(&choice);
    Ok(()) // returning nothing, aka unit type in Ok()
}

fn main() {
    // Method 1
    let choice: Result<Menu, _> = get_choice("main");

    match choice {
        Ok(choice) => print_choice(&choice),
        Err(e) => println!("error: {:?}", e),
    }

    // Method 2
    pick_choice("start");

    // Results challenge

    let customer = Customer {
        name: String::from("Jason"),
        age: 12,
    };
    let can_customer_buy: Result<(), String> = can_buy(&customer);
    println!("{:?}", can_customer_buy);

    match can_customer_buy {
        Ok(_) => println!("Can buy"),
        Err(e) => println!("{}", e),
    }

    // Result and question mark operator challenge
    let maintenance = Employee {
        emp_type: Employees::Maintenance,
        active: true,
    };
    let enter_maintenance = try_enter(&maintenance);
    let marketing = Employee {
        emp_type: Employees::Marketing,
        active: true,
    };
    let enter_marketing = try_enter(&marketing);
    let manager = Employee {
        emp_type: Employees::Manager,
        active: false,
    };
    let enter_manager = try_enter(&manager);
    let supervisor = Employee {
        emp_type: Employees::Supervisor,
        active: true,
    };
    let enter_supervisor = try_enter(&supervisor);
    let kitchen = Employee {
        emp_type: Employees::Kitchen,
        active: true,
    };
    let enter_kitchen = try_enter(&kitchen);
    let tech = Employee {
        emp_type: Employees::Tech,
        active: true,
    };
    let enter_tech = try_enter(&tech);

    println!("{:?}", enter_maintenance);
    println!("{:?}", enter_marketing);
    println!("{:?}", enter_manager);
    println!("{:?}", enter_supervisor);
    println!("{:?}", enter_kitchen);
    println!("{:?}", enter_tech);
}

// Results challenge

#[derive(Debug)]
struct Customer {
    name: String,
    age: i32,
}

fn can_buy(customer: &Customer) -> Result<(), String> {
    if customer.age >= 21 {
        Ok(())
    } else {
        Err("Customer is under 21".to_owned())
    }
}

// Result and question mark operator challenge

#[derive(Debug)]
enum Employees {
    Maintenance,
    Marketing,
    Manager,
    Supervisor,
    Kitchen,
    Tech,
}

#[derive(Debug)]
struct Employee {
    emp_type: Employees,
    active: bool,
}

fn try_enter(emp: &Employee) -> Result<(), String> {
    match emp.active {
        true => match emp.emp_type {
            Employees::Maintenance => Ok(()),
            Employees::Marketing => Ok(()),
            Employees::Manager => Ok(()),
            Employees::Supervisor => Err("No access".to_owned())?,
            Employees::Kitchen => Err("No access".to_owned())?,
            Employees::Tech => Err("No access".to_owned())?,
        },
        false => Err("Inactive card".to_owned()),
    }
}

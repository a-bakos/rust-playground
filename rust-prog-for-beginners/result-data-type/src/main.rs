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
}

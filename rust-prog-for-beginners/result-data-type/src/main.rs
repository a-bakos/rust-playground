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

fn main() {
    let choice: Result<Menu, _> = get_choice("main");

    match choice {
        Ok(choice) => print_choice(&choice),
        Err(e) => println!("error: {:?}", e),
    }
}

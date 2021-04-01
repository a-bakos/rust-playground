use std::env;
use std::fs;
use std::io::Read;

fn main() {
    // Require min 2 args to open the program
    if env::args().len() <= 1 {
        println!("Program requires at least arguments!");
        return;
    }

    let file_to_open = env::args().nth(1).unwrap();
    let name_to_find = env::args().nth(2).unwrap();

    // Read the file
    let contents = fs::read_to_string(&file_to_open).unwrap();
    let mut name_found = false;
    for line in contents.lines() {
        if line == name_to_find {
            name_found = true;
        }
    }

    if name_found == true {
        println!("Name is on the list!");
    } else {
        println!("Name is NOT on the list!");
    }
}

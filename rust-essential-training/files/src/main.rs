use std::env;

fn main() {
    if env::args().len() <= 2 {
        println!("Program requires at least 2 args");
        return;
    }

    for (index, argument) in env::args().enumerate() {
        println!("Argument {} is {}", index, argument);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
}

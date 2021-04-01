use std::{env, fs};

fn main() {
    // Number of args
    // if env::args().len() <= 2 {
    //     println!("Program requires at least 2 args");
    //     return;
    // }

    for (index, argument) in env::args().enumerate() {
        println!("Argument {} is {}", index, argument);
    }

    //let arg2 = env::args().nth(2).unwrap();
    //println!("arg2 is {}", arg2);

    // Read a file
    let contents = fs::read_to_string("series.txt").unwrap();
    println!("Content is {}", contents);
}

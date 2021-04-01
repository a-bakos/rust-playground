use rand::prelude::*;
use std::io;

fn main() {
    // we don't know how long the user input string will be
    // but okay, cos String type is stored on the heap, so
    // it can change dynamically.
    let mut buffer = String::new();

    println!("Enter a message: ");
    io::stdin().read_line(&mut buffer);
    println!("Buffer is {}", buffer);

    // if we want to parse the string as int
    // parse with turbofish operator or directly cast the var as i32
    // let number = buffer.trim().parse::<i32>(); // turbofish
    let number: i32 = buffer.trim().parse().unwrap();
    println!("number + 1 is {}", number + 1);

    let rand_number = random::<f64>(); // turbofish operator
    println!("number is {}", rand_number);

    let rand_gen = thread_rng().gen_range(1..11);
    println!("Number is {}", rand_gen);
}

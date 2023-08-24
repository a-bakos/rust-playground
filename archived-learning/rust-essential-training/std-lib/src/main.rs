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

    /**
     * Challenge: higher or lower
     */
    let mut user_choice = String::new();
    let computers_number = random::<i32>();
    println!("Guess the number!");

    loop {
        println!("Enter your guess:");
        io::stdin().read_line(&mut user_choice);
        let user_choice: i32 = user_choice.trim().parse().unwrap();

        println!("Your choice: {}", user_choice);

        if user_choice > computers_number {
            println!("Too big, try again.");
            continue;
        } else if user_choice < computers_number {
            println!("Too small, try again.");
            continue;
        } else {
            println!("That is correct, you win!");
            break;
        }
    }
}

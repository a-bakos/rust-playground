use rand::prelude::*;
use std::io;

fn main() {
    /**
     * Challenge: higher or lower
     */
    let computers_number: i32 = thread_rng().gen_range(0..1000);
    println!("Guess the number between 0 and 1000!");

    loop {
        let mut user_choice = String::new();
        println!("Enter your guess:");
        io::stdin().read_line(&mut user_choice);
        let user_choice: i32 = user_choice.trim().parse().unwrap();

        println!("Your choice: {}", user_choice);
        println!("COMPUTER: {}", computers_number);

        if user_choice > 1000 || user_choice < 0 {
            println!("You broke the rules!");
            break;
        }

        if user_choice > computers_number {
            println!("Too big, try again.");
        } else if user_choice < computers_number {
            println!("Too small, try again.");
        } else {
            println!("That is correct, you win!");
            break;
        }
    }
}

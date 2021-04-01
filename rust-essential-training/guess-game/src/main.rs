use rand::prelude::*;
use std::io;

fn main() {
    /**
     * Challenge: higher or lower
     */
    let computers_number: i32 = thread_rng().gen_range(0..1000);
    println!("Guess the number between 0 and 1000! You have 7 tries. And don't break the rules!");
    let mut rounds: i32 = 0;

    loop {
        if rounds == 7 {
            println!("You have no more rounds left!");
            println!("It was {}.", computers_number);
            break;
        }

        let mut user_choice = String::new();
        println!("Enter your guess:");
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line.");
        let user_choice: i32 = user_choice.trim().parse().unwrap();

        println!("Your choice: {}", user_choice);

        if user_choice > 1000 || user_choice < 0 {
            println!("You broke the rules!");
            break;
        }

        if user_choice > computers_number {
            rounds += 1;

            println!("Too big, try again.");
        } else if user_choice < computers_number {
            rounds += 1;
            println!("Too small, try again.");
        } else {
            println!("That is correct, you win!");
            break;
        }
    }
}

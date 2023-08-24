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
        let guess = match io::stdin().read_line(&mut user_choice) {
            Ok(_) => match user_choice.trim().parse::<i32>() {
                Ok(value) => value, // success
                Err(_) => {
                    println!("\nFailed to parse input. Guess again!");
                    continue;
                }
            },
            Err(_) => {
                println!("\nFailed to read input, Guess Again!");
                continue;
            }
        };

        println!("Your choice: {}", guess);

        if guess > 1000 || guess < 0 {
            println!("You broke the rules!");
            break;
        }

        if guess > computers_number {
            rounds += 1;

            println!("Too big, try again.");
        } else if guess < computers_number {
            rounds += 1;
            println!("Too small, try again.");
        } else {
            println!("That is correct, you win!");
            break;
        }
    }
}

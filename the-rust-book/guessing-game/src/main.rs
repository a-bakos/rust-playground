use rand::Rng; // Random Range
use std::cmp::Ordering;
use std::io; // Standard Library Input/Output

fn main() {
    println!("[ GUESS THE NUMBER! ]"); // println! is a macro call

    let sercret_number: u32 = rand::thread_rng().gen_range(1, 100);

    // println!("The secret number is: {}", sercret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadow the previous value and handle invalid input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // "_" underscore is a catch-all value
                println!("Enter a number, not a string");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        // Compare
        match guess.cmp(&sercret_number) {
            // Match arms:
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    println!("[ Good bye ]");
}

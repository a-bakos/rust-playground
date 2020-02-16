fn main() {
    println!("Hello, loop test!");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        println!("Round {}", counter);

        if counter == 5 {
            break counter * 2;
        }
    };

    println!("The result is doubled: {}", result);
}

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

    println!("[Loop] The result is doubled: {}", result);

    let mut iteration = 3;

    while iteration != 0 {
        println!("Iteration run");

        iteration = iteration - 1;
    }

    println!("[While] finished!");

    let collection = ['a', 'b', 'c', 'd', 'e'];

    for element in collection.iter() {
        println!("The value of this iteration is {}", element);
    }

    println!("[For] Finished");

    for number in (1..10).rev() {
        println!("{}", number);
    }

    println!("[For] Rev finished");
}

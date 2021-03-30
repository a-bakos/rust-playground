#![allow(unused_doc_comments)]
fn main() {
    let mut count = 0;

    // LOOP assigned to a variable and with a return value
    let result = loop {
        if count == 10 {
            // return the value of loop at break
            break count * 10;
        };

        count += 1;
        println!("count is {}", count);
    };
    println!("the result of loop is {}", result);

    /**
     * Key difference between loop {} and while {} is
     * that the break; statement in loop {} is used to
     * return a value.
     * The break; in while {} does not have the same
     * ability to return a value.
     */
    // WHILE loop
    // while evaluates the condition before each iteration
    // common use is to iterate through an array
    let mut count_2 = 0;

    while count_2 < 10 {
        count_2 += 1;
        println!("while {}", count_2);
    }

    count_2 = 0;
    let letters = ['a', 'b', 'c', 'd'];

    while count_2 < letters.len() {
        println!("letter is {}", letters[count_2]);
        count_2 += 1;
    }

    // FOR loop
    // designed to repeat a block of code for each
    // item in a collection such as an array

    let message = ['h', 'e', 'l', 'l', 'o'];
    for item in message.iter() {
        println!("message item is {}", item);
    }
    // if we need to know the index of the item,
    // use the .enumerate() method. This will create
    // a tuple with a pair of values.
    // So we need to destructure the tuple to
    // capture its components as individual variables.
    // We also need to borrow 'item' to get its value in the block
    for (index, &item) in message.iter().enumerate() {
        println!("message position {} item is {}", index, item);

        if item == 'e' {
            break;
        }
    }

    // For loop to iterate over a range of values
    for number in 0..=10 {
        println!("range loop {}", number);
    }
}

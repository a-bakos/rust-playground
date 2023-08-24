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

    /**
     * NESTED LOOPS
     */
    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t", num);
        }
        println!();
    }

    /**
     * Loops challenge
     */
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f32;

    min = numbers[0];
    max = numbers[0];
    mean = 0.0;
    for &number in numbers.iter() {
        if number > max {
            max = number;
        } else if number < min {
            min = number;
        }
        mean += number as f32;
    }
    mean /= numbers.len() as f32;

    println!("Min: {}", min);
    println!("Max: {}", max);
    println!("Mean: {}", mean);

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}

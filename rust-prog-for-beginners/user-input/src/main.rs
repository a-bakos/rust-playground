use std::io;

// This Result type is different in the io module because there's so many things
// can go wrong with io, the Err type is already defined, so only the
// success return type is needed
fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    // get the standard input, borrow buffer and save the input in buffer
    // read_line needs a mutable borrow to modify the buffer
    // The question mark operator indicates to use that the may possibly fail.
    // If if does fail, we'll return an Err type from the function and the
    // data will not have been read from the terminal.
    io::stdin().read_line(&mut buffer)?;
    // We'll trim the input becase the enter is included in the input by default
    // Then we take ownership again, because trim() returns a slice
    Ok(buffer.trim().to_owned())
}

fn main() {
    let mut all_input = vec![];
    let mut times_input = 0;

    while times_input < 2 {
        match get_input() {
            Ok(words) => {
                all_input.push(words);
                times_input += 1;
            }
            Err(e) => println!("Error {:?}", e),
        }
    }

    for input in all_input {
        println!(
            "Original {:?}, capitalized {:?}",
            input,
            input.to_uppercase()
        );
    }
}

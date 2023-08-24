#![allow(unused_doc_comments)]
fn main() {
    let mut message = String::from("Earth"); // Allocated on the heap
    println!("{}", message);
    message.push_str(" is home");
    println!("{}", message);

    // String ownership
    let rocket_fuel = String::from("RP-1");
    // Shadow the prev variable
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("Rocket_fuel is {}", rocket_fuel);

    let fuel_len = borrow_fuel(&rocket_fuel);
    println!("Borrowed fuel length is {}", fuel_len);

    fn process_fuel(mut propellant: String) -> String {
        println!("Processing propellant {}", propellant);
        propellant.push_str(" V2");
        // we're transferring ownership back
        propellant
    }

    // Borrowing
    fn borrow_fuel(propellant: &String) -> usize {
        println!("Borrowing propellant");
        let length = propellant.len();
        length
    }

    // Mutable borrowing
    let mut rocket_fuel_2 = String::from("RP-2");
    fn mutable_borrow_fuel(propellant: &mut String) -> usize {
        propellant.push_str(" + Added bit here");
        let length = propellant.len();
        length
    }
    mutable_borrow_fuel(&mut rocket_fuel_2);
    println!("After processing rocket fuel 2: {}", rocket_fuel_2);

    /**
     * Slices
     */
    let new_message = String::from("Greetings from Earth!");
    let earth_slice = &new_message[15..]; // from the 15th character to the end
    println!("The slice is {}", earth_slice);

    // In addition to slicing string,
    // we can also create slices that reference other types
    // of collections such as arrays:as
    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets: &[i32] = &planets[..4];
    println!("Inner planets are: {:?}", inner_planets);

    /**
     * &String != &str
     *
     * A borrowed &String reference will point to an actual
     * string on the stack which in turns points to and own
     * the string data that lives on the heap. String also
     * stores the length and the capacity.
     *
     * &str slice only stores the pointer to the heap data
     * along with the length. Doesn't need to know the
     * capacity because it doesn't own the string on the
     * heap.
     */
    fn get_first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        println!("{:?}", bytes);
        for (index, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..index]; // found a space
            }
        }

        &s // no spaces, this is a single word
    }
    let new_string = String::from("Hello Earth!");
    println!("{}", get_first_word(&new_string));

    /**
     * STRINGS Challenge
     * Trim whitespaces from front and back
     */
    let test1 = "We need more space.";
    println!("{}", trim_spaces(test1));
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("    There's space in front.");
    println!("{}", trim_spaces(&test2));
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear.     ");
    println!("{}", trim_spaces(&test3));
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = "    We're surrounded by space!      ";
    println!("{}", trim_spaces(test4));
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "           ";
    println!("{}", trim_spaces(test5));
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    println!("{}", trim_spaces(test6));
    assert_eq!(trim_spaces(test6), "");
}

fn trim_spaces(s: &str) -> &str {
    // locate the first non-space character
    let mut start = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }
    }

    // search in reverse to locate the last non-space character
    let mut end = 0;
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }

    &s[start..end]
}

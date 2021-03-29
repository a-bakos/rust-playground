fn main() {
    let a = 10.0;
    let b = 3.0;
    let c = a / b;

    print!("A is {}", a); // simple print macro
    print!("\n");

    println!("C is {}", c);
    println!("C is {:.3}", c); // control precision
    println!("C is {:8.3}", c); // total char spaces (padding with spaces)
    println!("C is {:08.3}", c); // total char spaces (padding with leading zeros to make a total of 8 chars)

    println!("C is {} and A is {}", c, a); // print multi vars
    println!("C is {0:.2} and A is {1} and once again, C is {0:.2}", c, a); // print multi vars multiple times by referencing their position

    let mut binary_value = 0b_1111_0101_u8;
    println!("Value is {}", binary_value);
    // print in binary format, where b is to print in binary, 8 to say how many bits, and 0 is to show leading zeros
    println!("Value is {:08b}", binary_value);

    // bitwise negation
    binary_value = !binary_value;
    println!("Value is {:08b}", binary_value);

    // bitwise AND to clear the bit at position 3
    binary_value = binary_value & 0b_1111_0111;
    println!("Value is {:08b}", binary_value);

    // bitwise OR
    binary_value = binary_value | 0b_0100_0000;
    println!("Value is {:08b}", binary_value);

    // bitwise XOR - exclusive OR
    // simple thinking: A = true, B = False -> XOR is True if A and B are different
    binary_value = binary_value ^ 0b_0101_0101;
    println!("Value is {:08b}", binary_value);

    // bitwise SHIFT
    binary_value = binary_value << 4;
    println!("Value is {:08b}", binary_value);
    binary_value = binary_value >> 4;
    println!("Value is {:08b}", binary_value);

    // CHAR data type
    let new_char = '\u{261D}';
    println!("This char: {}", new_char);
}

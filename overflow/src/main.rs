fn main() {
    println!("Hello, integer overflow error!");

    let integer: u8 = 255;

    let integer = integer + 1;

    // Attempt to add with overflow
    println!("{}", integer);
}

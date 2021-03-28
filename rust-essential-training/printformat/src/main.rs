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
}

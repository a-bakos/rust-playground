use std::io::{Read, Write};
use std::{env, fs};

fn main() {
    // Number of args
    // if env::args().len() <= 2 {
    //     println!("Program requires at least 2 args");
    //     return;
    // }

    for (index, argument) in env::args().enumerate() {
        println!("Argument {} is {}", index, argument);
    }

    //let arg2 = env::args().nth(2).unwrap();
    //println!("arg2 is {}", arg2);

    // Read a file
    let contents = fs::read_to_string("series.txt").unwrap();
    println!("Content is {}", contents);

    for line in contents.lines() {
        println!("{}", line);
    }

    let contents = fs::read("series.txt").unwrap(); // returns vector

    // This will print the individual bytes from the file
    // as unsigned integers
    println!("{:?}", contents); // Debug formatter

    /**
     * WRITE TO A FILE
     */
    let mut to_write = String::new();
    to_write.push_str("X-Files\n");
    to_write.push_str("House MD\n");
    to_write.push_str("Fringe\n");

    fs::write("series.txt", to_write);

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("series.txt")
        .unwrap();

    // mark this string literal to be interpreted as
    // a collection of byte values -- write doesn't care
    // what the data is, but it think of it as a series of
    // bytes so it expect an array of u8 values
    file.write(b"\nSuperstore");
}

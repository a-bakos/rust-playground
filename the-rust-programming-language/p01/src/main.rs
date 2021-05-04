// the compiler will ignore the error
#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem;

fn main() {
    println!("Hello, world!"); // println! is not a function, it's a macro

    let x = 0; // unused variable aka dead code

    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );
}

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

    let char_d: char = 'd'; // 32 bit unicode character
    println!(
        "char_d = {}, takes up {} bytes",
        char_d,
        mem::size_of_val(&char_d)
    );

    // f32 f64 IEEE754 signed!

    let a = 3;
    let a_cubed = i32::pow(a, 3);
    let b = 2.5;
    let b_cubed = f64::powi(b, 3); // i in powi stands for integer
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
}

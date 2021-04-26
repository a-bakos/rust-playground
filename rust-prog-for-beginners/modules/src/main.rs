use std::collections::HashMap;

mod greet {
    // need to define use statements in mods locally
    use std::collections::HashMap;

    fn hello() {
        println!("Hello");
    }

    fn goodbye() {
        println!("Bye");
    }
}

mod maths {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    // Two ways:
    // 1
    greet::hello();
    // 2
    use greet::*;
    hello();
}

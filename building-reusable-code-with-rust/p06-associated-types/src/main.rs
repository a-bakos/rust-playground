/// Associated types
/// vs Generics and Trait inheritance
/*
 * Associated types give you a way to specify
 * a placeholder type in the function signature.
 * So the implementor of a trait can specify
 * the concrete type to use.
 */
use std::fmt;

trait Iterator {
    type Item: fmt::Display; // associated type and forced display trait - trait bounds
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter;

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // implementation
        Some(42)
    }
}

fn main() {
    let mut counter = Counter {};

    let n_int: Option<u32> = counter.next();
    println!("{:?}", n_int);

    let n_unknown = counter.next();
    println!("{:?}", n_unknown);
}

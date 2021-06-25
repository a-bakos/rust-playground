fn main() {
    let s = "  hello  ";
    let p = s.trim();
    // At this point, 's' still exists.
    // It acts as a pointer to a piece of data
    // in memory that is never going to change.
    // Then 'p' is just a subset of 's'.

    println!("p = '{}'", p);
}

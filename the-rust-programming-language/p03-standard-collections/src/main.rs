fn main() {
    let mut mynums = Vec::new();
    mynums.push(1);
    mynums.push(2);
    mynums.push(3);

    let index: usize = 0; // note, this is not an i32/64
    mynums[index] = 4;
    println!("{:?}", mynums[index]);

    match mynums.get(5) {
        Some(x) => println!("X is there = {:?}", x),
        None => println!("No such element"),
    }

    mynums.push(100);
    let last_elem = mynums.pop(); // Get last elem, remove it from Vec, and store it in last_elem as an Option
    match last_elem {
        Some(elem) => println!("This was the last element {:?}", elem),
        None => println!("This was an empty vector..."),
    }
}

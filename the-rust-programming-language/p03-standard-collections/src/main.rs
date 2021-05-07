fn main() {
    let mut mynums = Vec::new();
    mynums.push(1);
    mynums.push(2);
    mynums.push(3);

    let index: usize = 0; // note, this is not an i32/64
    mynums[index] = 4;
    println!("{:?}", mynums[index]);
}

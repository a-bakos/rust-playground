fn main() {
    // Map
    let numbers = vec![1, 2, 3, 4];
    let plus_one = numbers.iter().map(|x| x + 1);
    plus_one.for_each(|x| println!("{}", x));

    println!("");

    // filter
    let numbers_2 = vec![1, 2, 3, 4];
    let larger_than_two = numbers_2.into_iter().filter(|&x| x > 2);
    larger_than_two.for_each(|x| println!("{}", x));
}

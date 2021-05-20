fn main() {
    /**
     * Generator adaptors
     */
    // Map
    let numbers = vec![1, 2, 3, 4];
    let plus_one = numbers.iter().map(|x| x + 1);
    plus_one.for_each(|x| println!("{}", x));

    println!("");

    // filter
    let numbers_2 = vec![1, 2, 3, 4];
    let larger_than_two = numbers_2.into_iter().filter(|&x| x > 2);
    larger_than_two.for_each(|x| println!("{}", x));

    println!("");

    /**
     * Consuming adaptors
     */
    let numbers_3 = vec![1, 2, 3, 4];
    let sum: i32 = numbers_3.iter().sum();
    println!("Sum: {}", sum);

    let max: &i32 = numbers_3.iter().max().unwrap();
    println!("Max: {}", max);
}

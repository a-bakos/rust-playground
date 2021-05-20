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

    // Max returns an option so need to unwrap
    let max: &i32 = numbers_3.iter().max().unwrap();
    println!("Max: {}", max);

    // Fold to do aggregation on some complex logic
    let numbers_4 = vec![1, 2, 3, 4];
    let fold_sum: i32 = numbers_4.iter().fold(0, |sum, val| sum + val);
    // fold:
    // 	 param 1 = initial value
    // 	 param 2 = a closure with two params:
    //		- accumulator
    //		- element
    println!("Fold sum: {}", fold_sum);
}

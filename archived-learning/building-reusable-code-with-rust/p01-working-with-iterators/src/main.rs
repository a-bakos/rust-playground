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

    // Lazy evaluation demonstration
    let numbers_5 = vec![1, 2, 3, 4];
    // Not consumed:
    // numbers_5.iter().map(|x| x + 1);

    let plus_one_iter = numbers_5.iter().map(|x| x + 1);
    println!("{:?}", plus_one_iter);

    let plus_one: Vec<i32> = plus_one_iter.collect();
    println!("{:?}", plus_one);

    // Because of lazy evaluation, we can
    // create a vec from 1 to infinity, but
    // only work with the elements the code
    // actually uses
    let numbers_6: Vec<i32> = (1..) // 1 to infinity
        .map(|x| x + 1) // [2, 3,  4,  5, ...]
        .map(|x| x * x) // [4, 9, 16, 25, ...]
        .take(5) // "take" the first five
        .collect(); // collect them into a vec

    println!("{:?}", numbers_6);
}

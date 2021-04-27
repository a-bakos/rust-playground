fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // method #1
    let mut plus_one = vec![];
    for num in numbers {
        plus_one.push(num + 1);
    }

    // better method #2
    let numbers2 = vec![1, 2, 3, 4, 5];
    let plus_one2: Vec<_> = numbers2.iter().map(|num| num + 1).collect();
}

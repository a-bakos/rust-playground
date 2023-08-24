fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // method #1
    let mut plus_one = vec![];
    for num in numbers {
        plus_one.push(num + 1);
    }

    // better method #2
    let numbers2 = vec![1, 2, 3, 4, 5];
    // _ means "some kind of a vector"
    let plus_one2: Vec<_> = numbers2.iter().map(|num| num + 1).collect();

    // filter
    let numbers3 = vec![1, 2, 3, 4, 5];
    // _ means "some kind of a vector"
    let new_nums: Vec<_> = numbers3.iter().filter(|num| num <= 3).collect();

    // find
    let numbers4 = vec![1, 2, 3, 4, 5];
    // _ means "some kind of a vector"
    let find_me: Option<i32> = numbers3.iter().find(|num| num == 3);

    // count
    let count = numbers.iter().count();

    // last
    // the reason last returns an option is
    // because numbers can be an empty vector
    let last: Option<i32> = numbers.iter().last();

    // min / max
    let min: Option<i32> = numbers.iter().min();
    let max: Option<i32> = numbers.iter().max();

    // take
    // take the first X number of items
    let take: Option<i32> = numbers.iter().take(3).collect();

    let data = vec![1, 2, 3, 4, 5];
    let times_three: Vec<_> = data
        .iter()
        .map(|num| num * 3)
        .filter(|num| num > &10)
        .collect();
    for num in times_three {
        println!("{}", num);
    }
}

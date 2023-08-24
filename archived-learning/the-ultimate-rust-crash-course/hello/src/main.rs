use hello::greet;

fn main() {
    greet();

    let random_num: u8 = rand::random();
    println!("{:?}", random_num);

    for num in [7, 8, 9].iter() {
        println!("{:?}", num);
    }

    let list = [("a", 1), ("b", 2), ("c", 3)];
    for (letter, num) in list.iter() {
        println!("{:?} : {:?}", letter, num);
    }
}

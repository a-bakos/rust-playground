// The best candidates to ues generics for
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
///// These could be rewritten as :
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    //                ^ Copy

    for &item in list.iter() {
        // ^ copy
        if item > largest {
            //  ^ PartialOrd::gt()
            largest = item;
        }
    }
    largest
}
// Another way to structure the function signature is:
// fn largest<T>(list: &[T]) -> T
//  where T: PartialOrd + Copy {}

struct PointInt {
    x: i32,
    y: i32,
}
struct PointFloat {
    x: f32,
    y: f32,
}
///// These could be rewritten as :

fn main() {
    let number_list = vec![34, 50, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // With generics
    let number_list = vec![34, 50, 100, 65];
    let result = largest::<i32>(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest::<char>(&char_list);
    println!("The largest char is {}", result);

    let integer = PointInt { x: 5, y: 10 };
    let float = PointFloat { x: 3.0, y: 9.0 };
}

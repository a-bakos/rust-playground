fn main() {
    let mut arr = ['a', 'b', 'c'];
    arr[0] = 'x';
    let first_letter = arr[0];
    println!("The first letter is {}", first_letter);

    // define empty i32 array with 5 elements then init values
    let new_arr: [i32; 5];
    new_arr = [0; 5];
    println!("The list item is {}", new_arr[4]);
    let index: usize = new_arr.len();
    println!("The length of the array is {}", index);

    // Create a 3 dimensional array full of 0-s
    let garage = [[[0; 100]; 20]; 5];

    let mut stuff: (i8, f32, char) = (4, 3.14, 'x');
    let first_tuple_item = stuff.0;
    stuff.1 = 6.28;
    println!(
        "First tuple item: {} / And second tuple item; {}",
        first_tuple_item, stuff.1
    );

    // Tuple destructuring
    let (a, b, c) = stuff;
    println!("Value of b is {}", b);
}

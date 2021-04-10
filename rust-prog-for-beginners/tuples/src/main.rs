fn get_coords(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}

fn main() {
    let coord_x: i32 = 5;
    let coord_y: i32 = -4;

    let (_x, y) = get_coords(coord_x, coord_y);

    if y > 5 {
        println!("Greater than");
    } else if y < 5 {
        println!("Less than");
    } else {
        println!("Equal");
    }
}

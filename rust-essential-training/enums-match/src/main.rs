// Enum = enumeration
#[derive(Debug)]
enum Shape {
    Circle(f64),             // radius
    Rectangle(f64, f64),     // width, height
    Triangle(f64, f64, f64), // side a b c
}

fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("My shape is {:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("This is a circle with radius {}", r), // Circle can store 1 val to represent its radius, so capture it with "r"
        Shape::Rectangle(w, h) => println!(
            "This is a rectangle with width of {} and height of {}",
            w, h
        ),
        Shape::Triangle(a, b, c) => println!("This is a triangle with sides {}, {}, {}", a, b, c),
    }

    let my_number = 4u8;
    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => {
            // default case
            println!("some number");
            "returned str slice"
        }
    };
    println!("Result is {}", result);
}

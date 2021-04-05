// Enum = enumeration
#[derive(Debug)]
enum Shape {
    Circle(f64),             // radius
    Rectangle(f64, f64),     // width, height
    Triangle(f64, f64, f64), // side a b c
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
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

    let perimeter = my_shape.get_perimeter();
    println!("Perimeter is {}", perimeter);

    // Option Enum
    let countdown = [5, 4, 3, 2, 1];
    //let number = countdown[5]; // doesnt exist
    let number = countdown.get(5); //(5) => None
                                   //let number = number.unwrap() + 1; // discouraged
    let number = number.unwrap_or(&0) + 1; // unwrap_or takes a parameter to return when the Options is None
    println!("Number is {:?}", number);
}

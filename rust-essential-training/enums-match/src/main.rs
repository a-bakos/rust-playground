// Enum = enumeration
#[derive(Debug)]
enum Shape {
    Circle,
    Rectangle,
    Triangle,
}

fn main() {
    let my_shape = Shape::Rectangle;
    println!("My shape is {:?}", my_shape);
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}
#[derive(Debug)]
struct Sqaure {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}
impl Shape for Sqaure {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn main() {
    println!("Hello, world!");
}

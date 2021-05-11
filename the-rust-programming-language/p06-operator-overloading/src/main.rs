// Operator overloading

use std::ops::Add;

#[derive(Debug)]
struct Complex<T> {
    real: T,
    imaginary: T,
}

impl<T> Complex<T> {
    fn new(real: T, imaginary: T) -> Complex<T> {
        Complex::<T> { real, imaginary } // need to use :: here so it doesn't get treated like "less than"
    }
}

// A)
// impl Add for Complex<i32> {
// B)
impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    // Associated type:
    type Output = Complex<T>;

    // Trying to support operations like a + b:
    // param1 self is "a", aka left-hand side. self is a reference
    // param2 is "b". Self (cap S) is the type of Complex
    // Self::Output is the associated type
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real, // == basically a T + T operation
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

fn main() {
    let mut a = Complex::new(1.0, 2.0);
    let mut b = Complex::new(3.0, 4.0);
    println!("{:?}", a + b);
}

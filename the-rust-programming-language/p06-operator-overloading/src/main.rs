// Operator overloading

use core::ops::AddAssign;
use core::ops::Neg;
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

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imaginary += rhs.imaginary;
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imaginary: -self.imaginary,
        }
    }
}

fn main() {
    let mut a = Complex::new(1.0, 2.0);
    let mut b = Complex::new(3.0, 4.0);
    println!("{:?}", a + b);

    // AddAssign
    let mut c = Complex::new(1.0, 2.0);
    let mut d = Complex::new(3.0, 4.0);
    c += d; // we need a trait for this to work
    println!("{:?}", c);

    // Negation
    let mut e = Complex::new(1.0, 2.0);
    println!("{:?}", -e);
}

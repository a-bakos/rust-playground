/*
   Dispatch meaning:
   How does the compiler figure out what to call
   There are different ways to dispatch on an
   invocation
   - Static: things are already pre-determined
   - Dynamic: to be determined at runtime

   Static is by definition more efficient.
*/

trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("String: {}", *self)
    }
}

// This creates monomorphisation.
// At compilation it creates concrete implementations of
// print_it for whatever types you request it.
fn print_it<T: Printable>(x: T) {
    println!("{}", x.format());
}

fn main() {
    let a: i32 = 123;
    let b: String = "hello".to_string();

    print_it(a);
    print_it(b);
}

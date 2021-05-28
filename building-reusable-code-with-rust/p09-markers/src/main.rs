#[derive(Debug)]
pub struct MyStruct {}

// Markers:
// Markers are empty traits that have no methods
// Implementing them on types will tell the compiler
// about some special properties they have.

// pub trait Copy: Clone {} // Clone is a supertrait of Copy

// Copy is trivial bitwise copy, can't be overloaded
// Notice it's empty, it's called a marker trait
// We need this to tell the compiler that we need
// the Copy
impl Copy for MyStruct {}

// Clone is explicit memory copy, usually expensive
impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        *self
    }
}

fn main() {
    let x = MyStruct {};
    let y = x;
    // y is a copy of x, x is NOT moved
    println!("{:?}", x);
}

// Because Copy and Clone are very trivial
// implementations, we can have the compiler
// implement them for us by using derive
//
// With derive, the compiler tries to write code
// for us.
#[derive(Debug, Copy, Clone)] // automatically implemented by the compiler
struct MyStruct2 {}

// Sized trait - vs - not sized
struct HasSizedT<T>(T); // T: Sized by default
struct NotSizedT<T: ?Sized>(T); // Negation with question mark

// [i32] is not Sized
struct NoOK(HasSizedT<[i32]>); // [i32] has no constant size known at compile time

struct OK(NotSizedT<[i32]>);

// op param will be a closure function
// for this, we need a Box with a standard
// function type <Fn>
// Unpacking the functions signature
// - math function takes a and b params as i32s
// - takes op, and the type of it is Box
// - the data type within the Box is Fn(i32, i32) which is a closure(!)
// - this closure accepts arguments and returns a value
// - dyn keyword signifies to the compiler that the
// Fn(i32, i32) -> i32 data could be different things
// dyn: "hey you might get multiple different kinds that are in this Box instead of just this one specific closure"
fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b) // i32
}

fn main() {
    let add = |a, b| a + b; // this is a closure
                            // place the closure into a box

    // we do this in order to be able to call the
    // closure from a function because the function
    // parameter need to have a known size.
    // Closure sizes can vary from small to large,
    // and in order to call function we have to know
    // exactly what the size of the function argument is
    // When we place the closure in a Box,
    // the Box is always guaranteed to be the same size
    // as a pointer which is the size of a memory address.
    // That's why we're able to pass the Box/pointer
    // to the function argument. Then that pointer can
    // point to any size of data.
    let add: Box<_> = Box::new(add); // with fn annotation

    // test the fn
    println!("{}", math(2, 2, add));

    let sub = Box::new(|a, b| a - b);
    let mul = Box::new(|a, b| a * b);
    println!("{}", math(2, 2, sub));
    println!("{}", math(2, 2, mul));
}

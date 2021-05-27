// Make MyNumber and i32 inter-convertible

#[derive(Debug)]
struct MyNumber {
    value: i32,
}

// from i32 to MyNumber
impl From<i32> for MyNumber {
    fn from(number: i32) -> Self {
        MyNumber { value: number }
    }
}

// When we implement From, we get
// the corresponding Into:
// impl Into<MyNumber> for i32 - for free

fn main() {
    // convert an i32 into MyNumber
    let num = MyNumber::from(42_i32);
    println!("{:?}", num);
}

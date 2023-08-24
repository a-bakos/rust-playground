// i32 implements Copy marker trait
// it means it'll automatically copy all of the memory across

#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

fn main() {
    let p = Person {
        name: "Frank".to_string(),
        age: 6,
    };

    // if the struct doesn't have a clone marker trait implemented,
    // now we'd have two variables pointing to the same memory, aka p2 = p
    let p2 = p.clone();

    println!("p = {:?}, p2 = {:?}", p, p2);
}

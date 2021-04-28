trait Noise {
    // traits take a self ref
    fn make_noise(&self);
}

fn hello(noisy: impl Noise) {
    noisy.make_noise();
}

// for traits we need struct-s or enum-s
struct Person;
impl Noise for Person {
    // fn signature must match that of the trait
    fn make_noise(&self) {
        println!("Hello hello");
    }
}

struct Dog;
impl Noise for Dog {
    fn make_noise(&self) {
        println!("Woof woof");
    }
}

trait Perimeter {
    fn calc_perimeter(&self) -> i32;
}
fn perimeter(thing: impl Perimeter) {
    println!("{:?}", thing.calc_perimeter());
}

struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}
impl Perimeter for Triangle {
    fn calc_perimeter(&self) -> i32 {
        self.a + self.b + self.c
    }
}

struct Square {
    a: i32,
}
impl Perimeter for Square {
    fn calc_perimeter(&self) -> i32 {
        self.a * 4
    }
}

fn main() {
    hello(Person {});
    hello(Dog {});

    perimeter(Triangle { a: 1, b: 2, c: 3 });
    perimeter(Square { a: 10 });
}

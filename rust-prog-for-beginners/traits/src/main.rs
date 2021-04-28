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

fn main() {
    hello(Person {});
    hello(Dog {});
}

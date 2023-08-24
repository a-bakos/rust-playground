/// Storing objects of different types in a vector
/// Method 1: using an enum
/// Method 2: using Box
use core::fmt::Debug;

trait Animal {
    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

#[derive(Debug)]
struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

#[derive(Debug)]
struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

// Method 1
enum Creature {
    Human(Human),
    Cat(Cat),
}

fn main() {
    // Method one
    let mut creatures: Vec<Creature> = Vec::new();
    creatures.push(Creature::Human(Human { name: "Tom" }));
    creatures.push(Creature::Cat(Cat { name: "Frank" }));

    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }

    // Method 2
    let mut animals: Vec<Box<Animal>> = Vec::new();
    animals.push(Box::new(Human { name: "Tom" }));
    animals.push(Box::new(Cat { name: "Frank" }));

    for a in animals.iter() {
        a.talk()
    }
}

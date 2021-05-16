/// Storing objects of different types in a vector
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

enum Creature {
    Human(Human),
    Cat(Cat),
}

fn main() {
    let mut creatures: Vec<Creature> = Vec::new();
    creatures.push(Creature::Human(Human { name: "Tom" }));
    creatures.push(Creature::Cat(Cat { name: "Frank" }));

    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }
}

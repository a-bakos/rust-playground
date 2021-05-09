trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn new(name: &'static str) -> Self {
        Self { name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn new(name: &'static str) -> Self {
        Self { name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

fn main() {
    let human = Human { name: "Frank" };
    human.talk();

    let cat = Cat { name: "Felix" };
    cat.talk();

    let human2 = Human::new("Grace");
    human2.talk();

    let cat2 = Cat::new("Molly");
    cat2.talk();
}

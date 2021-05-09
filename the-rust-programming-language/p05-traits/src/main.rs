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
    fn new(name: &'static str) -> Human {
        Human { name }
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
    fn new(name: &'static str) -> Cat {
        Cat { name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

fn main() {
    // Method 1

    let human = Human { name: "Frank" };
    human.talk();

    let cat = Cat { name: "Felix" };
    cat.talk();

    // Method 2

    let human2 = Human::new("Grace");
    human2.talk();

    let cat2 = Cat::new("Molly");
    cat2.talk();

    // Method 3 -- Using Animal to create, and the type of the variable
    // helps the compiler decide if it's a Human or a Cat

    let human3: Human = Animal::new("Kelly");
    human3.talk();

    let cat3: Cat = Animal::new("Doge");
    cat3.talk();
}

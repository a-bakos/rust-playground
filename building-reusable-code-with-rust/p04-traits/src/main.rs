trait Vehicle {
    // if no &self param => this is a static method
    fn new(name: &'static str) -> Self; // static method

    // if &self param => this is an instance method
    fn self_move(&self) -> (); // instance method
}

struct Airplane {
    name: &'static str,
}
struct Bicycle {
    name: &'static str,
}
struct Car {
    name: &'static str,
}

impl Vehicle for Airplane {
    fn new(name: &'static str) -> Self {
        Airplane { name }
    }

    fn self_move(&self) {
        self.fly();
    }
}
impl Airplane {
    fn fly(&self) {
        println!("Flying");
    }
}

impl Vehicle for Bicycle {
    fn new(name: &'static str) -> Self {
        Bicycle { name }
    }

    fn self_move(&self) {
        self.pedal();
    }
}
impl Bicycle {
    fn pedal(&self) {
        println!("Pedalling");
    }
}

impl Vehicle for Car {
    fn new(name: &'static str) -> Self {
        Car { name }
    }

    fn self_move(&self) {
        self.drive();
    }
}
impl Car {
    fn drive(&self) {
        println!("Flying");
    }
}

fn main() {
    println!("Hello, world!");
}

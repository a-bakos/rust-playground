use std::rc::Rc;

struct Person {
    name: Rc<String>,
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person { name }
    }

    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

fn main() {
    let name = Rc::new("Frank".to_string());
    println!(
        "Name = {}, name has {} strong pointers.",
        name,
        Rc::strong_count(&name)
    );
    {
        let person = Person::new(name.clone());
        println!(
            "Name = {}, name has {} strong pointers.",
            name,
            Rc::strong_count(&name)
        );
        person.greet();
    }
    println!(
        "Name = {}, name has {} strong pointers.",
        name,
        Rc::strong_count(&name)
    );
}

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
    /// Rc is a very useful class for sharing around a single variable in
    /// many locations in the code without worrying about all the borrowing
    /// and ownership effects, but it is limited to just a single thread!
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

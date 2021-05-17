//! Hello documentation
//!
//! This is your program.

use std::sync::Arc;
use std::thread;

struct Person {
    name: Arc<String>,
}

impl Person {
    fn new(name: Arc<String>) -> Person {
        Person { name }
    }

    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

/// If you plan to not just pass around a variable in a single thread, but
/// pass it around in many threads, instead of Rc, you want Arc.
/// By Atomic we mean the changes to the number of references are guarded,
/// they are protected so as different threads start and stop using the
/// variable ("name" here) they would change the variable which keeps the
/// number of references, the number of pointers, they would do it in a
/// thread safe way.
fn main() {
    let name = Arc::new("Frank".to_string());
    let person = Person::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    });

    println!("name = {}", name);

    t.join().unwrap();
}

use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Cat {
    name: &'static str,
    breed: &'static str,
    age: u8,
}

impl Display for Cat {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{} the {}-years old {} cat",
            self.name, self.age, self.breed
        )
    }
}

fn main() {
    let a_cat = Cat {
        name: "Frank",
        breed: "tuxedo",
        age: 6,
    };

    // Display:
    println!("{}", a_cat);

    // Debug
    println!("{:?}", a_cat);

    // Debug with pretty-print
    println!("{:#?}", a_cat);
}

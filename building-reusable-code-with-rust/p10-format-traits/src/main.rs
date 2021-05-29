use std::fmt::{Display, Formatter, Result};

// If you have very complex data structures, you can consider implementing
// Debug yourself

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

    // DEFAULT TRAIT
    // use default values for all the fields
    let data1: MyData = Default::default();
    println!("{:?}", data1);

    // specify part of the fields and use the defaults for the rest
    let data_2 = MyData {
        int_field: 42,
        ..Default::default() // use the default for the rest
    };
    println!("{:?}", data_2);

    let default_pet: Pet = Default::default();
    println!("{:?}", default_pet);
}

// Default trait
// Helpful when we want to define default values

#[derive(Default, Debug)]
struct MyData {
    int_field: i32,
    float_field: f32,
}

// Defaults can also be used on enums
#[derive(Debug)]
enum Pet {
    Cat,
    Dog,
    Bird,
}

impl Default for Pet {
    fn default() -> Pet {
        Pet::Cat
    }
}

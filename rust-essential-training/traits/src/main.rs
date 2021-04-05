// DERIVE:
// when you derive traits, the compiler will
// generate default code for the required methods.
#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32, // miles
}

trait Description {
    fn describe(&self) -> String {
        // Default trait implementation
        String::from("This is an object flying through space!")
    }
}

impl Description for Satellite {
    // If the implementation block is empty, the default
    // trait implementation will be used!
    // fn describe(&self) -> String {
    //     format!(
    //         "The {} flying at {} miles per second!",
    //         self.name, self.velocity
    //     )
    // }
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "The {} flying at {} miles high with {} crew members aboard!",
            self.name, self.altitude, self.crew_size
        )
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };

    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254,
    };

    println!("{}", hubble.describe());
    println!("{}", iss.describe());

    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42,
    };

    // Equal only if all of the fields are equal
    println!("Hubble == GPS is {}", hubble == gps);
    // Hubble will be greater because it first compares
    // the name field (order specified on struct implementation)
    // and Hubble's name is longer than GPS
    println!("Hubble == GPS is {}", hubble > gps);

    // Trait bounds
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    print_type([13]);

    // Multiple trait bounds
    compate_and_print(1.0, 1);
    compate_and_print(1.1, 1);
}

// Trait bounds

use std::any;
use std::fmt;

fn print_type<T: fmt::Debug>(item: T) {
    println!("{:?} is {:?}", item, any::type_name::<T>());
}

// Multiple trait bounds

fn compate_and_print<T, U>(a: T, b: U)
where
    T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display + PartialEq + Copy,
{
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

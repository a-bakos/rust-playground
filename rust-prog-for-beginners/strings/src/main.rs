struct Item {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("Name: {}", name);
}

fn main() {
    let receipt = vec![
        Item {
            name: "Almonds".to_owned(),
            count: 1,
        },
        Item {
            name: String::from("Rice milk"),
            count: 5,
        },
    ];

    for item in receipt {
        print_name(&item.name);
        println!("Count: {}", item.count);
    }

    // challenge
    let people = vec![
        Person {
            age: 7,
            name: String::from("Jonah"),
            color: String::from("Blue"),
        },
        Person {
            age: 8,
            name: String::from("Dinah"),
            color: String::from("Green"),
        },
        Person {
            age: 9,
            name: String::from("Amy"),
            color: String::from("Red"),
        },
        Person {
            age: 22,
            name: String::from("Glenn"),
            color: String::from("White"),
        },
    ];

    for person in people {
        if person.age <= 10 {
            person.name_and_color();
        }
    }
}

// Challenge

struct Person {
    age: i8,
    name: String,
    color: String,
}

impl Person {
    fn name_and_color(&self) {
        println!("Name is {}, and fav color is {}", self.name, self.color);
    }
}

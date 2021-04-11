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
}

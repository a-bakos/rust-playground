use std::collections::HashMap;

// In hashmaps, the entries are not stored in order.
// This differs from vectors.

#[derive(Debug)]
struct Contents {
    content: String,
}

fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(
        1,
        Contents {
            content: "stuff".to_owned(),
        },
    );
    lockers.insert(
        2,
        Contents {
            content: "keys".to_owned(),
        },
    );
    lockers.insert(
        3,
        Contents {
            content: "bag".to_owned(),
        },
    );

    for (key, value) in lockers.iter() {
        println!("{} {:?}", key, value);
    }
}

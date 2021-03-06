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

    // Hashmap challenge
    let mut stock = HashMap::new();
    stock.insert(String::from("Chair"), 5);
    stock.insert(String::from("Bed"), 3);
    stock.insert(String::from("Table"), 2);
    stock.insert(String::from("Couch"), 0);

    let mut total_stock: i32 = 0;
    for (furniture, on_stock) in stock.iter() {
        total_stock = total_stock + on_stock;
        match on_stock {
            0 => println!("{} - Out of stock", furniture),
            _ => println!("{}: {}", furniture, on_stock),
        }

        // Another way to print
        // Need to borrow the 0 zero because when we iterate through a
        // hashmap, the key and value are automatically borrowed
        let stock_count = if on_stock == &0 {
            "Out of stock".to_owned()
        } else {
            // Format macro turns it into a String
            format!("{:?}", on_stock)
        };
        println!("item={:?}, count={:?}", furniture, stock_count);
    }

    println!("Total stock items: {}", total_stock);
}

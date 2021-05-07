fn main() {
    let mut mynums = Vec::new();
    mynums.push(1);
    mynums.push(2);
    mynums.push(3);

    let index: usize = 0; // note, this is not an i32/64
    mynums[index] = 4;
    println!("{:?}", mynums[index]);

    match mynums.get(5) {
        Some(x) => println!("X is there = {:?}", x),
        None => println!("No such element"),
    }

    mynums.push(100);

    println!("{:?}", mynums);

    let last_elem = mynums.pop(); // Get last elem, remove it from Vec, and store it in last_elem as an Option
    match last_elem {
        Some(elem) => println!("This was the last element {:?}", elem),
        None => println!("This was an empty vector..."),
    }

    println!("{:?}", mynums);

    // Read this like:
    // If mynums.pop() yields a value (x) of Some,
    // get the value and print it.
    if let Some(x) = mynums.pop() {
        println!("{:?}", x);
    }

    println!("{:?}", mynums);

    /////////////////////////
    /// HashMap
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("A triangle has {} sides", shapes["triangle"]);

    for (key, value) in &shapes {
        println!("{} : {}", key, value);
    }

    // Do we have a circle entry?
    // If not, create one with the value 1,
    // otherwise, don't do anything.
    shapes.entry("circle".to_string()).or_insert(1);

    ////////////////////////
    // HashSet
    // represents a mathematical set,
    // only unique elements,
    // no order guarantee

    let mut greeks = HashSet::new();
    greeks.insert("alpa");
    greeks.insert("gamma");
    greeks.insert("delta");

    println!("{:?}", greeks);

    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("Added vega");
    }

    if !greeks.contains("kappa") {
        println!("we dont have kappa");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("delta removed");
    }

    // Set operations
    let set_1_5: HashSet<_> = (1..=5).collect(); // use _ for type inference
    let set_6_10: HashSet<_> = (6..=10).collect();
    let set_1_10: HashSet<_> = (1..=10).collect();
    let set_2_8: HashSet<_> = (2..=8).collect();

    // subset - every single element of a set is part of another set
    println!(
        "Is {:?} a subset of {:?}? {}",
        set_2_8,
        set_1_10,
        set_2_8.is_subset(&set_1_10)
    );

    // disjoint - no common elements, all sets are independent
    println!(
        "Is {:?} a disjoint of {:?}? {}",
        set_1_5,
        set_6_10,
        set_1_5.is_disjoint(&set_6_10)
    );

    // union - both of the sets
    // intersection - elements that are present in both sets
    println!(
        "items in {:?} and {:?}: {:?}",
        set_2_8,
        set_6_10,
        set_2_8.union(&set_6_10)
    );

    // difference = items in the first set but not in second
    // symmetric difference = items in the union but not in the intersection (union minus intersection)
}

use std::collections::HashMap;
use std::collections::HashSet;

use std::collections::HashMap;

fn main() {
    let mut friends: Vec<String> = Vec::new();
    friends.push(String::from("Joey"));
    friends.push(String::from("Ross"));
    friends.push(String::from("Chandler"));
    friends.push(String::from("Gunther"));

    println!("Friends are: {:?}", friends);

    let last = friends.pop();
    println!("Last friend is {:?}", last);
    //let fourth = &friends[3];
    //println!("Try to get Gunther {}", fourth); // out of bounds error
    let fourth = friends.get(3);
    println!("Try to get Gunther: {:?}", fourth); // None

    println!("Friends are: {:?}", friends);

    let third = &friends[2];
    println!("The third friend is {}", third);

    // Pre-populate the vector with values:
    let countdown = vec![5, 4, 3, 2, 1];
    println!("{:?}", countdown);

    /**
     * Hashmaps
     *
     * Orderkeys stored are not necessarily the same as you inserted them/
     */
    let mut superstore_crew_salary = HashMap::new(); // data type not defined
    superstore_crew_salary.insert("Amy", 119000);
    superstore_crew_salary.insert("Jonah", 23000);
    superstore_crew_salary.insert("Dina", 30000);
    superstore_crew_salary.insert("Cheyenne", 26000);
    superstore_crew_salary.insert("Cheyenne", 28000); // overwriting
    superstore_crew_salary.entry("Dina").or_insert(45000); // insert if it doesn't exist
    println!("Crew salary: {:?}", superstore_crew_salary);

    let jonah = superstore_crew_salary.get("Jonah"); // returns option enum
    println!("Jonah: {:?}", jonah);

    // Updating HashMap entries
}

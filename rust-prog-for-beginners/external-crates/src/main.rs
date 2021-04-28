use chrono::prelude::*;

fn main() {
    println!("Hello, world!");
    let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
    println!("UTC = {:?}", utc);
    let local: DateTime<Local> = Local::now();
    println!("LOCAL = {:?}", local);
}

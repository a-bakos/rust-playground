// &'static str
// static means that the variable will live as long as the program lives

// Example 1 - where we absolutely need to specify the lifetime

struct Person {
    name: String,
}

// 'a hints to Rust that ceo Person will live as long as Company does.
// Otherwise, Person (boss) could be invalidated/destroyed before Company (company)
// does and therefore it would have an invalid ceo reference.
struct Company<'a> {
    name: String,
    ceo: &'a Person,
}

fn main() {
    let boss = Person {
        name: String::from("Tony Stark"),
    };
    let company = Company {
        name: String::from("Stark Industries"),
        ceo: &boss,
    };
}

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

// Lifetime elision
impl Person {
    // When you write this:
    // fn get_ref_name(&self) -> &String {
    // ... this is what you're actually getting by the compiler (it does if for
    // you in the background, makes sure all parts will live long enough):
    fn get_ref_name<'a>(&'a self) -> &'a String {
        &self.name
    }
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

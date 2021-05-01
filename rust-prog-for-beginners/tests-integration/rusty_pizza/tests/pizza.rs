use rusty_pizza::Pizza;

#[test]
fn can_make_pepperoni_pizza() {
    let pizza = Pizza::pepperoni(12);
    assert_eq!(pizza.topping, "pepperoni");
    assert_eq!(pizza.inches, 12);
}

#[test]
fn can_make_mozzarella_pizza() {
    let pizza = Pizza::mozzarella(16);
    assert_eq!(pizza.topping, "mozzarella");
    assert_eq!(pizza.inches, 16);
}

// Now that our integration tests setup is done, we can execute the cargo test
// command to see the results
// Inspecting the output reveals that Rust places each kind of test in a
// different section: unit tests, integration tests, and documentation tests.
// In the integration tests section, we can see that our two tests inside the
// tests/pizzas.rs file were collected and executed by the test suite.
// Only library crates can be tested via integration tests because binary crates
// don't expose any functionality that other crates can use. But, many Rust
// crates that provide a binary have a succinct src/main.rs file that calls
// logic that lives in a src/lib.rs file. Using that structure, integration
// tests can test the important parts of the code.

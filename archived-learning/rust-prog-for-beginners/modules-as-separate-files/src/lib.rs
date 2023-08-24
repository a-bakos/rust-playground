// Add pub to mod's so they're accessible in
// demo.rs
pub mod group; // C)
pub mod helpers; // A)

pub fn print_from_lib() {
    // B)
    use helpers::{print_again, print_from_helper};

    println!("Print from lib");

    // A)
    helpers::print_from_helper();
    // B)
    print_again();

    // C)
    group::g1::g1_hello();
}

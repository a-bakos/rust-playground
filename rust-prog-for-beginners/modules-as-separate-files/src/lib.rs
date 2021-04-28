mod helpers; // A)

pub fn print_from_lib() {
    // B)
    use helpers::{print_again, print_from_helper};

    println!("Print from lib");

    // A)
    helpers::print_from_helper();
    // B)
    print_again();
}

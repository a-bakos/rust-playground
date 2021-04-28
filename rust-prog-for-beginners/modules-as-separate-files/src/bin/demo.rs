// the name "demo" is specified in the cargo.toml file
use demo::group;
use demo::print_from_lib;

fn main() {
    println!("Hello");
    print_from_lib();

    // this is available here because of pub mods in lib
    group::g1::g1_hello();
}

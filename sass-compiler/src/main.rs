use std::fs;
use std::io::{Read, Write};

fn main() {
    let entry_file = fs::read_to_string("style.scss").unwrap();
    println!("Content is {}", entry_file);

    let mut css_file = String::new();
    css_file.push_str(".row{background:red;}");

    fs::write("style.css", css_file);
}

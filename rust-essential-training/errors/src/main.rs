use std::fs;

fn main() {
    //panic!("PROBLEM!");
    // Enable Backtrace:
    // Type this in the console
    // $Env:RUST_BACKTRACE=1
    // $Env:RUST_BACKTRACE=0 // Disable

    // Recoverable errors:
    // Missing file
    let content = fs::read_to_string("some_file.txt");
    println!("content is {:?}", content);

    // File exists
    // Unwrap is simple but will panic and fail if the file doesn't exist
    //let content_exists = fs::read_to_string("hello.txt").unwrap();
    let content_exists = fs::read_to_string("hello.txt");
    println!("content from hello is {:?}", content_exists);
}

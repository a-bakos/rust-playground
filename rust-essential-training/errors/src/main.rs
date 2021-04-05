use std::fs;
use std::io;

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

    // Better error handling:
    // Use match
    let result = fs::read_to_string("hello.txt"); // Result enum
    let contents = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found"),
            io::ErrorKind::PermissionDenied => String::from("Permission Denied"),
            _ => panic!("Another type of error! {:?}", error),
        },
    };
    println!("Contents is {:?}", contents);

    // Propagating Errors
    let propagate_result = read_and_combine("file1.txt", "file2.txt");
    match propagate_result {
        Ok(s) => println!("Propagate result is:\n{}", s),
        Err(e) => println!("There was an error!{}", e),
    };
}

fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    // Shorthand to propagate
    // Only use the shorthand for functions that return
    // a Result enum
    let mut s1 = fs::read_to_string(f1)?;
    // Verbose way to propagate
    let s2 = match fs::read_to_string(f2) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}

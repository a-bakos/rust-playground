fn main() {
    let s = "  hello  ";
    let p = s.trim();
    // At this point, 's' still exists.
    // It acts as a pointer to a piece of data
    // in memory that is never going to change.
    // Then 'p' is just a subset of 's'.
    println!("p = '{}'", p);

    // String

    let ss = "  hello  ".to_string();
    let pp = ss.trim();
    // In this case, the data for 'ss' was copied
    // from the binary into a new place on the heap.
    // The memory that was part of the binary ("  hello  ") was copied onto the heap into a new place and now ss is pointing to that new place on the heap.

    // A String is a str in a Box!

    // Example
    let fstr = "help me find home";
    let found = string_find_f(fstr);
    println!("{:?}", found);
}

fn string_find_f(s: &str) -> &str {
    for (n, x) in s.char_indices() {
        if x == 'f' {
            return &s[n..];
        }
    }
    s
}

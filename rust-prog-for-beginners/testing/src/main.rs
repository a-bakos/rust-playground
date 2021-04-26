fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    // in Rust, a collection of code is called
    // a crate, and crate here refers to the top
    // of the (current) code (above)
    use crate::*;

    #[test]
    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from("HELLO");
        assert_eq!(result, expected, "Needs to be uppercase!");
    }
}

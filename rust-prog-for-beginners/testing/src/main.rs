fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

/// ensures n is >= lower and <= upper
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn concat(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
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

    #[test]
    fn clamp_lower() {
        let result = clamp(10, 100, 1000);
        let expected = 100;
        assert_eq!(result, expected, "Should be 100");
    }

    #[test]
    fn clamp_higher() {
        let result = clamp(5000, 100, 1000);
        let expected = 1000;
        assert_eq!(result, expected, "Should be 1000");
    }

    #[test]
    fn check_div() {
        let result = div(1, 1);
        let expected = Some(1);
        assert_eq!(result, expected, "Should be 1");
    }

    #[test]
    fn check_div_zero() {
        let result = div(1, 0);
        let expected = None;
        assert_eq!(result, expected, "Should be None");
    }

    #[test]
    fn check_concat() {
        let result = concat("a", "b");
        let expected = "ab".to_owned();
        assert_eq!(result, expected, "Should be ab");
    }
}

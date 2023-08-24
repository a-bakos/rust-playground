fn main() {
    // Option combinators

    let a: Option<i32> = Some(1); // Change this to None to see the differences in evaluations
    let a_is_some = a.is_some(); // T or F
    dbg!(a_is_some);
    let a_is_none = a.is_none(); // T or F
    dbg!(a_is_none);

    // argument is the inner val of the Option;
    // it'll add 1 if we have a value Some, otherwise
    // nothing happens
    let a_mapped = a.map(|num| num + 1);
    dbg!(a_mapped);

    // param is the inner val of the Option;
    // closure body needs to return T or F
    // if T -> keep the data, if F throw it away
    // and becomes None
    let a_filtered = a.filter(|num| num == &1);
    dbg!(a_filtered);

    // closure with no arguments;
    // returns optional data
    // if A is data, nothing happens,
    // if A is no data, it'll return Some(5)
    let a_or_else = a.or_else(|| Some(5));
    dbg!(a_or_else);

    // closure with no arguments;
    // this won't return optional data;
    // if we A has data, it'll be the var's value
    // otherwise, 0 becomes the default value,
    // aka set a default value when data is not present
    let unwrapped = a.unwrap_or_else(|| 0);
    dbg!(unwrapped);
}

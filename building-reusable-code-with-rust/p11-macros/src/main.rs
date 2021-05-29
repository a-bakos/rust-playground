// METAPROGRAMMING
// Macros

// Macros are more complex and you probably don't need to use them unless you're
// running out of options.

// Create a custom macro
macro_rules! hello {
    () => {
        // matches no argument
        // all call to hello!() will be expanded to this:
        println!("hello");
    };
}

// Let's re-implement the folllowing vec! macro ourselves:
// let x: Vec<u32> = vec![1,2,3];
// -> expands to:
/*
let x: Vec<u32> = {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
};
*/

macro_rules! vec {
	( $($x:expr), * ) => {
		// block
		let mut temp_vec = Vec::new();
		$( temp_vec.push($x); )*
		temp_vec
	};
}

fn main() {
    hello!();
}

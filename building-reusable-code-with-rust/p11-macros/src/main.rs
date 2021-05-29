// METAPROGRAMMING
// Macros

// Macros are more complex and you probably don't need to use them unless you're
// running out of options.

// Macros are not included by default because they don't have namespaces so
// there could be code collisions.
// When you import a module/crate you have to explicitly say:
// #[macro_use]
// mod my_module_with_macros;
//
// Loads only my_macro! from the crate:
// #[macro_use(my_macro)]
// extern crate my_crate_with_macros

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

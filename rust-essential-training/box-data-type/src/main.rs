use std::mem;

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0,
    };
    println!(
        "Vehicle size on stack is {} bytes.",
        mem::size_of_val(&vehicle)
    );

    // Let's move this shuttle to store it on the heap instead
    // by putting it in a box.
    // Data type will be a Box, followed by Shuttle in angle brackets.
    // This tells Rust that the "thing" this box is pointing to
    // is a Shuttle. That way the compiler knows how to interpret
    // that data on the other end of the pointer and how
    // much space it will occupy on the heap.
    // This is NOT a copy operation: vehicle will lose ownership
    // of the struct. boxed_vehicle will become the new owner
    // of the pointer on the stack
    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    println!(
        "boxed_vehicle size on stack is {} bytes. (This is the size of the pointer to the data on the heap)",
        mem::size_of_val(&boxed_vehicle)
    );
    println!(
        "boxed_vehicle size on heap is {} bytes.",
        mem::size_of_val(&*boxed_vehicle)
    );

    // If we want to move that Box data back from the heap to the stack,
    // we can do so by dereferencing it.
    // This will place the struct back on the stack,
    // and pass ownership to the unboxed_vehicle variable
    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!(
        "unboxed_vehicle size on stack is {} bytes.",
        mem::size_of_val(&unboxed_vehicle)
    );
}

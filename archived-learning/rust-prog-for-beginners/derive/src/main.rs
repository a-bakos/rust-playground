// Derive is a special macro that is applied to enums and structs.
// With clone and copy: ownership is no longer transfered when you move
// the Position enum into a function or a structure, a copy is made instead.

// Note that the copy clone derive should not be used on large structs
// ~ up to 4-5 entries.

#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

// All fields withing this struct have to also derive the functionalities
#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_emp(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };

    println!("{:?}", me);

    // Observe the me variable won't change ownership,
    // a copy is made instead because of the copy clone derive
    // We'll have two copies
    print_emp(me);
    print_emp(me);
}

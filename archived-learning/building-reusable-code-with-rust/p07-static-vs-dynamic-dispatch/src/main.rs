// Static vs Dynamic Dispatch
// When we call a method on a trait, there are
// two things that could happen: static and dynamic
// dispatch.
//
// Static dispatch is very fast because it's known
// at compile time, which implementation to use.
// No runtime overhead, but bloated code.
//
// Dynamic dispatch - trait objects

trait ShowMyself {
    fn show(&self) -> String;
}

impl ShowMyself for u32 {
    fn show(&self) -> String {
        format!("I'm u32 {}", *self)
    }
}
impl ShowMyself for String {
    fn show(&self) -> String {
        format!("I'm String {}", *self)
    }
}

// This will generate a static dispatch
// because when we call it, we know it's gonna
// be a u32 or String so the compiler will know
// which implementation to use ...
fn show_myself<T: ShowMyself>(x: T) {
    println!("{:?}", x.show());
}
// ... So what the compiler will do is create
// separate implementations for both types:
// fn show_myself_u32(x: u32) { ... }
// fn show_myself_string(x: string) { ... }

fn main() {
    let x: u32 = 42;
    let y: String = "Hello, World".to_string();

    show_myself(x);
    show_myself(y);

    // ... compiler chooses the right implementation
    // and inlines it
    // show_myself_u32(x);
    // show_myself_u32(y);

    // Dynamic dispatch
    // (sidenote: two ways to use trait objects with
    // concrete types: specify the type or cast it
    // as trait object)
    // The problem with this is that the compiler
    // will not know which type implementation to use
    // and this will create a performance hit at
    // runtime.
    // Trait objects as implemented internally,
    // are structs, and contain a data field and a
    // vtable field. The first one points to the
    // concrete type data, the latter is a list of
    // function pointers.
    // Dynamic dispatch goes into this vtable field
    // to look up the function pointers and then calls
    // it as needed.
    let v: &ShowMyself = &42;
    let w = &"Hello, world".to_string() as &ShowMyself;
    show_myself_dyn(v);
    show_myself_dyn(w);
}

/// Dynamic dispatch (ignore the conflict)
impl ShowMyself for String {
    fn show(&self) -> String {
        format!("I'm a String {} -- from trait object", *self);
    }
}

fn show_myself_dyn(x: &ShowMyself) {
    println!("{}", x.show());
}

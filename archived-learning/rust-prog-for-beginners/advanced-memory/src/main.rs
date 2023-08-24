// An example how to utilise heap memory

enum Answer {
    Yes,
    No,
}

fn main() {
    let yes = Answer::Yes; // on the stack, but we want to store it on the heap:

    // in order to store it on the heap,
    // we have to create a Box. A Box is a pointer,
    // which is a memory address
    let yes_heap: Box<Answer> = Box::new(yes);
    // to access the data from the heap:
    let yes_stack = *yes_heap;
}

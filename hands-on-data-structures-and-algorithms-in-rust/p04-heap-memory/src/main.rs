// the main case for using heap memory is when you don't
// know how big your data structure is going to be.

#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    // make it a pointer:
    next: Option<Box<LinkedList<T>>>,
}

impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    let mut ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 2,
            next: None,
        })),
    };

    // mutable reference
    // Don't break apart the object to get me
    // the data I want, I only want a reference
    // to it.
    if let Some(ref mut v) = ll.next {
        v.add_up(10);
    }

    println!("{:?}", ll);
}

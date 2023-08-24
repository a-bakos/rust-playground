use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

fn main() {
    // Heap allocated pointers
    let mybox: Box<i32> = Box::new(42); // heap allocated
    let myrc: Rc<i32> = Rc::new(42); // reference counted

    // Cell with internal mutability
    let mycell: Cell<i32> = Cell::new(42);
    // mycell.set(0);
    // mycell.get();
    let myrefcell: RefCell<i32> = RefCell::new(42); // with read-write lock

    // myrefcell.borrow()
    // myrefcell.borrow_mut()

    // Thread-safe version
    let myarc: Arc<i32> = Arc::new(42); // atomic reference counter Rc<T>
    let mymutex: Mutex<i32> = Mutex::new(42);
    // mutex.lock()
    let myrwlock: RwLock<i32> = RwLock::new(42);
    // myrwlock.lock()

    // Composition examples:
    // Rc<RefCell<T>>
    // Rc<Vec<RefCell<T>>>
}

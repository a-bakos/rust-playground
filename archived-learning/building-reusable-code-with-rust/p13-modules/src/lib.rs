mod animal; // the content moved to animal/mod.rs
            // when rust sees the above, it'll try to find a file name animal.rs or a folder
            // named animal and a mod.rs file inside it.

// If your module doesn't contain sub-modules it'll be sufficient to just create
// a mod.rs file

fn main() {
    animal::move_around();
    animal::cat::meow();
}

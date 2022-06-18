CMD | DESC
--- | ----
`rustup update` | get the latest version
`rustc --version` | see the language version
`cargo --version` | see Cargo's version
`rustc main.rs` | Build a Rust file
`main.exe` | Run a Rust program
`cargo build`| Build with Cargo
`cargo run` | Build + run with Cargo
`cargo run -q` | Build + run with no console messages
`cargo run -p package_name` | Build + run specified package (used with Workspaces)
`cargo check` | Build check with Cargo
`cargo test` | Run code test (test mod's and #[cfg(test)])
`cargo test test_name` | Run specific a test by name
`cargo test --doc` | Further documentation for testing
`cargo test --help` | Displays the options that can be used with `test`
`cargo test -- --help` | Displays the options after the `--` separator
`cargo test -- --test-threads=1` | Run test without parallelism (one thread only)
`cargo test -- --nocapture` | Disable output capture (show printed values for passing tests)
`cargo test -- --ignored` | Run only ignored tests
`cargo build --release` | Build for release
`cargo clean` | Remove the `target/` folder
`cargo new project-name` | New Cargo project
`cargo new --lib my-lib` | Naw library file/project
`cargo update` | Update a crate (ignore lock)
`cargo doc --open` | Generate local documentation (use /// to comment)
`cargo tree` | Show dependency graph
`cargo clippy` | See idiomatic code suggestions
`cargo fmt` | Automatic code formatter
`cargo search crate-name` | Search for a crate on crates.io, eg. structopt
`cargo help` | Cargo command info
`rustup doc` | Open documentation
`rustdoc file.rs` | Generate local html documentation (into doc folder)

`rm -rf ~/.cargo/registry/index/*` | Remove cargo registry (fixes "Blocking waiting for file lock on package cache")

**Rust online playground** - https://play.rust-lang.org/

**Sublime Text 3: Rust Enhanced package** - https://github.com/rust-lang/rust-enhanced

**Coding Standards: RustFMT**

- https://github.com/rust-lang/rustfmt
- https://packagecontrol.io/packages/RustFmt

**Other**

- https://cheats.rs/
- https://soushi888.notion.site/Rust-b8318583755e49448ccd59784df08c62

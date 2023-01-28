CMD | DESC | MORE
--- | ---- | ----
`rustup update` | get the latest version |
`rustup component list --installed` | see installed Rust components | 
`rustup component add cmpnt-name` | install a component | e.g. rust-docs
`rustup toolchain list` | see what toolchains are installed |
`rustup docs --help` | see book docs options | 
`rustup docs --book` | open the book |
`rustup docs std` | open standard library documentation |
`rustup docs std::iter` | open iter module's doc in std library |
`rustc --version` | see the language version |
`cargo --version` | see Cargo's version |
`rustc main.rs` | Build a Rust file |
`main.exe` | Run a Rust program |
`cargo --list` | List Cargo commands (custom commands included) |
`cargo build`| Build with Cargo |
`cargo run` | Build + run with Cargo |
`cargo run -q` | Build + run with no console messages |
`cargo run -p package_name` | Build + run specified package in a workspace |
`RUST_BACKTRACE=1 cargo run` | Run with backtrace log |
`cargo check` | Build check with Cargo |
`cargo test` | Run code test (test mod's and #[cfg(test)]) |
`cargo test test_name` | Run specific a test by name OR test(s) that have this argument in their name |
`cargo test --doc` | Further documentation for testing |
`cargo test --help` | Displays the options that can be used with `test` (arguments for the test utility) |
`cargo test -- --help` | Displays the options after the `--` separator (arguments for the test binary) |
`cargo test -- --test-threads=1` | Run test without parallelism (one thread only) |
`cargo test -- --nocapture` | Disable output capture (show printed values for passing tests) |
`cargo test -- --show-output` | See everything that is printed to standard output
`cargo test -- --ignored` | Run only ignored tests |
`cargo test -- --test '*'` | Only run integration tests (not unit tests)
`cargo test --lib` | Only run unit tests (not integration tests)
`cargo test -p package_name` | Run tests for a particular package in a workspace |
`cargo build --release` | Build for release |
`cargo clean` | Remove the `target/` folder |
`cargo new project-name` | New Cargo project |
`cargo new --lib my-lib` | Naw library file/project |
`cargo update` | Update a crate (ignore lock) |
`cargo doc --open` | Generate local documentation (use /// to comment) |
`cargo doc --no-deps --open` | Generate doc for only your crate (no external crates)
`cargo tree` | Show dependency graph |
`cargo clippy` | See idiomatic code suggestions |
`cargo fmt` | Automatic code formatter |
`cargo search crate-name` | Search for a crate on crates.io, eg. structopt |
`cargo help` | Cargo command info |
`cargo add {crate-name}` | Add dependencies to a Cargo.toml manifest file | **[More](https://doc.rust-lang.org/cargo/commands/cargo-add.html)**
`rustup doc` | Open documentation |
`rustdoc file.rs` | Generate local html documentation (into doc folder) |

## Other commands

CMD | DESC
--- | ----
`rm -rf ~/.cargo/registry/index/*` | Remove cargo registry (fixes "Blocking waiting for file lock on package cache")

**Rust online playground** - https://play.rust-lang.org/

**Sublime Text 3: Rust Enhanced package** - https://github.com/rust-lang/rust-enhanced

**Coding Standards: RustFMT**

- https://github.com/rust-lang/rustfmt
- https://packagecontrol.io/packages/RustFmt

**Other**

- https://cheats.rs/
- https://soushi888.notion.site/Rust-b8318583755e49448ccd59784df08c62
- Let's get Rusty

### VS Code extensions

[Rust analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

[Code LLDB debugger](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

[Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)

[Error Lens - inline errors](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)

[Crates - crates.io support](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)

### Potential errors

Error  | **Corrupted file/component**    
-----  | ----------------------------
Msg    | `error: failed to install component: 'cargo-x86_64-pc-windows-msvc', detected conflict: 'lib/rustlib\manifest-cargo-x86_64-pc-windows-msvc'`
Step 1 | Remove toolchain: `rustup toolchain remove stable-x86_64-pc-windows-msvc`
Step 2 | Re-install toolchain: `rustup install stable-x86_64-pc-windows-msvc`


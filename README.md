CMD | DESC | MORE
--- | ---- | ----
`rustup update` | get the latest version |
`rustup component list --installed` | see installed Rust components | 
`rustup component add cmpnt-name` | install a component | e.g. rust-docs
`rustup toolchain list` | see what toolchains are installed |
`rustup toolchain install nightly --allow-downgrade` | install the nightly compiler | eg. for `cargo-expand` crate to expand macros
`rustup target list` | list of available compilation target environments
`rustup target install {target}` | install a specific build target | eg. `x86_64-apple-darwin`
`cargo +nightly expand` | use the nightly toolchain just for this command invocation
`rustup doc` | Open documentation |
`rustdoc file.rs` | Generate local html documentation (into doc folder) |
`rustup docs --help` | see book docs options | 
`rustup docs --book` | open the book |
`rustup docs std` | open standard library documentation | same: `rustup doc --std`
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
`cargo check --workspace` | Build check on the workspace | On ll crates included in the workspace in the manifest Cargo.toml
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
`cargo build --release --target x86_64-apple-darwin` | build for a specific target
`cargo clean` | Remove the `target/` folder |
`cargo new project-name` | New Cargo project |
`cargo new --lib my-lib` | Naw library file/project |
`cargo update` | Update a crate (ignore lock) |
`cargo doc --open` | Generate local documentation (use /// to comment) |
`cargo doc --no-deps --open` | Generate doc for only your crate (no external crates)
`cargo tree` | Show dependency graph |
`cargo clippy` | See idiomatic code suggestions |
`cargo clippy -- -D warnings` | fail clheck if clippy emits any warnings | useful in a CI pipeline
`cargo fmt` | Automatic code formatter | [fine tune formatting](https://github.com/rust-lang/rustfmt) in `rustfmt.toml`
`cargo fmt -- --check` | code formatting | in a CI pipeline
`cargo search crate-name` | Search for a crate on crates.io, eg. structopt |
`cargo help` | Cargo command info |
`cargo add {crate-name}` | Add dependencies to a Cargo.toml manifest file | **[More](https://doc.rust-lang.org/cargo/commands/cargo-add.html)**
`cargo install cargo-watch` | install cargo watch to monitor changes | 
`cargo install cargo-audit` | install cargo audit to scan dependency tree for reported security vulnerabilities | 
`cargo watch -x check` | run cargo watch | runs cargo check after every code change
`cargo watch -x check -x test -x run` | chaining processes | runs check, then tests, then launches application

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

Error  | **IntelliJ Idea format on save fails / rustfmt error / says old version but current version is used / async fn**    
-----  | ----------------------------
Msg    | `Execution failed (exit code 1). C:/[...]/.cargo/bin/rustfmt.exe --emit=stdout stdout : stderr : error[E0670]: `async fn` is not permitted in Rust 2015 --> <stdin>:3:1  3 [...] switch to Rust 2018 or later  = help: pass `--edition 2021` to `rustc` = note: for more on editions, read https://doc.rust-lang.org/edition-guide`
Step 1 | Outdated rustfmt version. Check: `rustfmt --version`. Update: `cargo install rustfmt`.
Step 2 | Rust edition not specified in rustfmt command. Try: `rustfmt --edition 2021`
Step 3 | Misconfigured IntelliJ IDEA settings. Adjust the command or the plugin's settings so the edition flag is passed as additional argument `--edition 2021`.

Error | **rust-dgb**
----- | ------------
Msg   | `error: the 'rust-gdb.exe' binary, normally provided by the 'rustc' component, is not applicable to the 'stable-x86_64-pc-windows-msvc' toolchain`
Info  | https://github.com/rust-lang/rustup/issues/2838
Info  | https://github.com/rust-lang/rustup/issues/2843
Fix   | ???

### Directives

What | How
---- | ---
Mute clippy warnings | `#[allow(clippy::lint_name)]`

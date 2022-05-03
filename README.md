# Learning Rust

A repository to track my progress learning Rust, primarily following 'The Rust Programming Language': https://doc.rust-lang.org/stable/book/

## Installing Rust

Instructions for installing Rust can be found <a href="https://doc.rust-lang.org/stable/book/ch01-01-installation.html">here</a>.

## Running Files in Rust

Rust uses the file extension .rs. To compile a file, we run `rustc [filename].rs`, and to run the resulting binary, we use `./[filename]`.

## Using Cargo to Build Projects

Cargo is a useful tool that handles various tasks like building your code and downloading the libraries that your code depends on.

### Initialising Cargo

To initialise Cargo in a new folder as a Git repository, run `cargo new [folder-name]`. You can add the flag `--vcs=none` (as in `cargo new --vcs=none [folder-name]`) to avoid creating the Git repository. Cargo can also be initialised in an existing folder using `cargo init`.

This creates a project with a `Cargo.toml` file which contains metadata for the project and keeps track of the dependencies used. It will also create a `main.rs` file in the `src` folder, which is the generally the entry point of a program.

A crate in Rust is a compilation unit. Note that a crate can be compiled into a binary or into a library. Libraries have no entry point, but instead provide functionality for other crates to use. To create a library, add the `--lib` flag to `cargo new`.

### Building a Project

To build an existing project, use `cargo build`. Cargo stores the result of the build in the `./target/debug directory`. If you want to compile and run the project, use `cargo run`.

Additionally, you can use `cargo build --release` to build for production.

### Other Cargo Options

You can run `cargo check` to check that the code compiles without building, and `cargo doc --open` to see generated documentation.

## Learning Resources

- <a href="https://doc.rust-lang.org/book/">The Rust Programming Language</a>: an introductory book about Rust
- <a href="https://doc.rust-lang.org/stable/rust-by-example/">Rust By Example</a>: examples illustrating various Rust concepts
- <a href="https://doc.rust-lang.org/stable/cargo/">The Cargo Book</a>: learn more about Cargo 
- <a href="https://github.com/rust-lang/rustlings">Rustlings</a>: small exercises for getting used to reading and writing Rust code
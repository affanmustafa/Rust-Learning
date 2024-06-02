This repository is for following along the official Rust book. I will be adding my notes and code snippets as I go through the book. 

This link for the book: [The Rust Programming Language](https://doc.rust-lang.org/book/)

## Notes
- Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.
- Rust is a statically typed language, which means that it must know the types of all variables at compile time.
- Rust is an ahead-of-time compiled language, which means you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.
- Rust is a great choice when performance is important and you want to be sure that a program doesn’t have any undefined behavior.
- Rust is also great when you need to call code written in another language.
- Rust’s rich type system and ownership model guarantee memory safety and thread safety.
- Rust has a powerful macro system that allows metaprogramming.
- Rust has a minimal runtime, which means you can write programs with very small binaries.
- Rust has a great build system and a great package manager - Cargo.
- Rust has a lot of documentation and a great community!
- Rust is statically typed and compiled ahead of time.

## Installation
- You can install Rust by running the following command in your terminal:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Cargo
- Cargo is Rust’s build system and package manager.

## Hello, World!
- To create a new project, you can run the following command:
```bash
cargo new hello_world
```
- This will create a new directory called `hello_world` with the following structure:
```
hello_world
├── Cargo.toml
└── src
    └── main.rs
```
- The `Cargo.toml` file is the configuration file for the project.
- The `src` directory contains the source code for the project.
- The `main.rs` file contains the main function, which is the entry point for the program.
- To build and run the project, you can run the following command:
```bash
cd hello_world
cargo run
```
- This will compile and run the project, and you should see the output `Hello, world!`.
- You can also build the project without running it by running the following command:
```bash
cargo build
```
- This will compile the project and create an executable in the `target/debug` directory.
- You can run the executable by running the following command:
```bash
./target/debug/hello_world
```
## To be added: 
Add Obsidian notes and code snippets. 


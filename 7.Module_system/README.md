# learn_rust_lets_get_rusty

## [Video 7: Rust's Module System Explained!](https://www.youtube.com/watch?v=5RPXgDQrjio&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=7)

### This chapter covers about the following

- [Modules](#modules)
- [Packages](#packages)
- [Crates](#crates)

## Modules

- Modules allow you to organize chunk of code and give privacy rules.
- Modules provide path which is used to call and used in other modules.
- Modules are created by using the `mod` keyword followed by the name of the module.
- Modules can contain other modules inside it. It can also contain functions, structs, enums, constants and traits.
- In Rust a Path can be `absolute` or a `relative`.
- Paths are specified by identifiers separated by `::`.
- By default, everything inside a parent module is private.
- Child modules can access anything defined inside the parent module.
- To make a module public, we need to add `pub` keyword.
- `super` keyword allows us to access the functions in parent module.
- `use` keyword allows us to bring a path into scope.
- To export or use a module or function inside a module into another module add `pub` keyword in front of the `use` keyword.

## Packages

- When we run `cargo new <name>` it creates a new package.
- Package stores `crates`.
- Rules:
  - A package must have one crate.
  - A package can have zero library crate or one library crate.
  - A package can have any number of binary crates.
- If we want more than one binary crate, we can create a folder named `bin` and add as many binary crate.

## Crates

- Two types of crates:
  - `binary` crate which gets executed. `cargo new <name>`.
  - `library` crate which gets used by other programs. `cargo new --lib <name>`
- Crates contain `modules`.
- When we create a package, inside `src` directory there is a `main.rs` file. Rust creates a binary crate with the name of the package automatically.
- If we create a `lib.rs` in the `src` directory then Rust creates a library crate automatically.

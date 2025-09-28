# learn_rust_lets_get_rusty

## [Video 16: Writing a CLI App in Rust - Part 2!](https://www.youtube.com/watch?v=AABHxixn6Cw&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=16)

### This chapter covers about the following

- [Create a Library crate](#create-a-library-crate)
- [Test Driven Development](#test-driven-development)
- []()

## Create a Library crate

- We will create a library crate inside `lib.rs` and move the `run` function and `config` struct.
- Make the functions and struct public so that we can import in main.rs.
- Move the required `use` statements.

## Test Driven Development

- Write test cases to handle search functionality.
- Write test cases to handle case sensitive and case insensitive query terms.
- Add `case_sensitive` as an attribute in Config struct.
- SET `CASE_INSENSITIVE` as an environment variable and read using `env`.
- use `eprintln!` to pass the output to the terminal instead of a file.

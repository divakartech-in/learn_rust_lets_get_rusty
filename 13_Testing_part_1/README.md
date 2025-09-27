# learn_rust_lets_get_rusty

## [Video 13: Testing in Rust!](https://www.youtube.com/watch?v=18-7NoNPO30&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=13)

### This chapter covers about the following

- [Writing Tests in Rust](#writing-tests-in-rust)

## Writing Tests in Rust

- In Rust, a function is a test if it has the `#[test]` attribute.
- We can also have helper functions within tests.
- We can run tests using `cargo test` command.
- Each test is run in a new thread and if the main thread sees the test thread failed then the tests are failed.
- We can use `assert!` macro to check for booleans and `assert_eq!` macro to compare two values and `assert_ne!` macro to compare two values are not equal.

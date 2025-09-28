# learn_rust_lets_get_rusty

## [Video 14: Testing in Rust - Part 2!](https://www.youtube.com/watch?v=-L4nKAlmH3M&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=14)

### This chapter covers about the following

- [Running Tests](#running-tests)
- [Integration Tests](#integration-tests)

## Running Tests

- All tests are executed in parallel in separate threads.
- All generated output are captured and not printed.
- We can set the number of threads to run the test by using the command line option `cargo test -- --test-threads=1`
- To display test outputs, use `cargo test -- --show-output`.
- We can run tests by using test name, `cargo test <test_name>`.
- To ignore a test from running, we can use `#[ignore]` attribute.

## Integration Tests

- In Rust, Unit tests are written in the same file as the product code.
- Integration tests are written inside a folder called `tests` at the root of the project.

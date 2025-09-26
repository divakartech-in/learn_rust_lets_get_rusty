# learn_rust_lets_get_rusty

## [Video 9: Error Handling in Rust!](https://www.youtube.com/watch?v=wM6o70NAWUI&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=9)

### This chapter covers about the following

- [Panic Macro](#panic-macro)
- [Result Enum](#result-enum)

## Panic Macro

- Panic macro is used to quit the program and throw an error message.
- If we run our program with `RUST_BACKTRACE=1 cargo run` then it will list all the function call till the error message for backtracking.
- Panic should be used only in exceptional cases. Always try to handle the error using Result Enum.

## Result Enum

- Result Enum will help to quit the program gracefully without panicking.
- Result will have two variants `Ok(T)` to handle `success` and `Err(E)` to handle failure cases.
- We can use the `match` statement to match both the variants and return the value accordingly.
- Error Propagation means returning the error back to the caller so that the caller function can take necessary steps.

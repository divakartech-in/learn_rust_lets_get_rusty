# learn_rust_lets_get_rusty

## [Video 12: Rust Lifetimes Finally Explained!](https://www.youtube.com/watch?v=juIINGuZyBc&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=12)

### This chapter covers about the following

- [Lifetimes](#lifetimes)
- [Generic Lifetime Annotations](#generic-lifetime-annotations)
- [Rules of Lifetime](#rules-of-lifetime)

## Lifetimes

- Lifetime means how long the variable lives for.
- Dingling References are references which point to invalid data.
- Rust Borrow checker runs at compile time and checks if all borrowed values or references are valid.

## Generic Lifetime Annotations

- Generic Lifetime Annotations describe the relationship between the lifetime of multiple references and how they relate to each other.
- These are specified by `'`.
- In a function return, The lifetime of the returned reference will be the same of the smallest lifetimes of the arguments.
- If we have passed 5 arguments to a function, the lifetime of the return value will be equal to the argument which has smallest lifetime.

## Rules of Lifetime

- Each parameter that is a reference gets its own lifetime paramter.
- If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
- If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` the lifetime of self is assigned to all output lifetime parameters.

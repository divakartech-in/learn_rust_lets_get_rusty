# learn_rust_lets_get_rusty

## [Video 3: Common Programming Concepts in Rust!](https://www.youtube.com/watch?v=2V0JaMVjzws&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=3)

### This chapter covers about the following

- [Create variables in Rust.](#create-variables-in-rust)
- [Create constants in Rust.](#create-constants-in-rust)
- [Shadowing in Rust.](#shadowing-in-rust)
- [DataTypes in Rust.](#datatypes-in-rust)
- [Functions in Rust.](#functions-in-rust)
- [Control Flow in Rust.](#control-flow-in-rust)
- [Comments in Rust.](#comments-in-rust)

## Create variables in Rust

- To create variables in Rust, we use `let` keyword. `let x = 5`.
- By default, all variables in Rust are immutable.
- To make it mutable, we should add `mut` keyword. `let mut x = 5`.

## Create constants in Rust

- Constants are immutable even if we `mut` keyword.
- In general, Constants are declared in all upper case and has to signed manually. `const COUNT: u32 = 10000`.
- Constants are assigned to constant expressions and cannot be returned from a function.
- We can use underscore in the values of constants. `const COUNT: u32 = 100_000`.

## Shadowing in Rust

- Instead of creating a mutable variable and updating the value of a variable. We can redeclare the variable again and provide a new value.

  ```
  let x = 5;
  println!("The value of x is: {}", x);
  let x = "six";
  println!("The value of x is: {}", x);
  ```

## DataTypes in Rust

- Scalar data types generally represents a single value.

  - Integers:
    - Integers are either signed or unsigned and by default integers are signed 32 bit.
    - Types of integer values.
      - Decimal
      - Hex
      - Octal
      - Binary
      - Byte
    - Integer overflow means Rust will panic in debug build if we set the value more than the max value an integer type can hold.
  - Floating point numbers:
    - Floating point numbers are by default 64 bit double precision.
    - We can do all basic numeric operations (addition, subtraction, multiplication, division, remainder).
  - Booleans - Can have two values `true` or `false`;
  - Characters - Can have individual characters.

- Compound data types represents a group of values.

  - Tuples
    - Comma separated list inside parenthesis.
    - We can destructure and get the values from a tuple.
    - We can also access the values of the tuple using index.
  - Arrays
    - Arrays in Rust are of fixed size by default.
    - If you want dynamic size, then we have to use a Vector.

## Functions in Rust

- Rust uses snake case or all lower case for function names with underscore. `my_function`.
- To return a value from a function, we can use `return` keyword and specify the return data type in the function.

## Control Flow in Rust

- If -> else if -> else statements, In rust the if statement conditions must be explicitly a boolean.
- Types of loops:
  - `Basic` loop is defined with the `loop` keyword and this loop will keep running until we explicitly call `break` statement.
  - `While` loop created with a condition and when the condition is satisfied the loop exits.
  - `For` loop created `for in` statement and iterated over an array using `iter()` method. We can also loop over a range of values using the for loop.

## Comments in Rust

- Three type of comments in Rust:
  - Line Comment are written using `//`
  - Block Comment are written using `/* */`
  - Document Comments

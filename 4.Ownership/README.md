# learn_rust_lets_get_rusty

## [Video 4: Understanding Ownership in Rust!](https://www.youtube.com/watch?v=VFIOSWy93H0&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=4)

### This chapter covers about the following

- [Ownership in Rust.](#ownership-in-rust)
- [References in Rust.](#references-in-rust)
- [Borrowing in Rust.](#borrowing-in-rust)
- [Slice Type in Rust.](#slice-type-in-rust)

## Ownership in Rust

- Ownership allows Rust to make memory safety programs without using a garbage collector.
- Ownership Rules:
  - Each value in Rust has a variable that's called its owner.
  - There can only be onw owner at a time.
  - When the owner goes out of scope, the value will be dropped.
- In Rust, For simple data types like Integer, float the memory is allocated in the stack, since their memory size is defined by the type signed/unsigned (8, 16, 32, 64 bits).
- But for dynamic data types like String, the memory is allocated in heap and the reference is stored in the stack along with the size and capacity.
- In languages like C, Allocation and Deallocation of memory on heap has to be done manually. But in Rust, It is done automatically when the variable goes out of scope.
- Copying the value of one variable to another is possible for simple data types since it is stored in stack but for complex data types it has to be either a `move` or `clone` operation.

- Ownership in Functions:
  - If you pass a variable to a function call without returning anything, then it is considered that the variable has moved out of scope and cannot be used inside the first function call. `Refer the code in main.rs`
  - We can take ownership of a variable and return it back using function calls.

## References in Rust

- Taking ownership and giving back is tedious process, but we still need to use the variable without taking ownership we use `references`.
- References are used with `&` in front of the variable.
- References are immutable by default. We need to use `mut` for the variable and pass `&mut`able reference to the function.
- Mutable references has a restriction, we can have ONLY one mutable reference for a variable within a scope.

## Borrowing in Rust

- Passing in references as function parameters is called `Borrowing`.

## Slice Type in Rust

- Slice provides reference to continuous sequence of elements in a collection.

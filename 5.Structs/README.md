# learn_rust_lets_get_rusty

## [Video 5: Structs in Rust!](https://www.youtube.com/watch?v=n3bPhdiJm9I&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=5)

### This chapter covers about the following

- [Structs](#structs)
- [Tuple Structs](#tuple-structs)
- [Unit Structs](#unit-structs)
- [Derived Traits](#derived-traits)

## Structs

- Structs are used to group related data.
- Structs help us create objects with attributes, each attributes can be of different data types.
- We can create an instance of the struct and access its attributes using `.` notation. `user.username`;
- We can mutate/modify the attributes of a struct instance. But we have to make the struct instance mutable.
- We can use attributes from one struct instance to another new struct instance.
- Methods are similar to functions, but instead of being generic it is tied to a particular struct. We need to use the `impl` keyword fot this.
- The first argument for a method inside `impl` is always the `&self`.
- Struct allows to create multiple implementation blocks.
- We can also create `associated functions` inside a `impl` block which is not tied to the struct.
- Associated functions first argument is not `&self`.

## Tuple Structs

- Structs without named fields are called `Tuple Structs`.

## Unit Structs

- Structs without any fields are called `Unit Structs`.

## Derived Traits

To print the details of custom structs, we need to add #[derive(Debug)] trait on top of the structs.

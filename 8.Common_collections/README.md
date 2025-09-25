# learn_rust_lets_get_rusty

## [Video 8: Common Collections in Rust!](https://www.youtube.com/watch?v=Zs-pS-egQSs&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=8)

### This chapter covers about the following

- [Vectors](#vectors)
- [Strings](#strings)
- [Hashmaps](#hashmaps)

## Vectors

- Vectors are created using `Vec<type>` keyword.
- An empty vector is initialized by using `Vec::new()`.
- We need to define the type while creating a vector.
- We can add values in a vector using `push` method.
- We can also create a Vector with values using `vec!` macro.
- Vectors will be dropped from the heap when the scope ends.
- Vector elements can be accessed by using its index.
- If the index is not present in the vector, it will throw a Index out of bounds error.
- To handle the error, we can use `match` with `Some` and `None`.
- To iterate over vector elements, we can use `for in`.
- Vectors can store only one type of data. If we want to store different types we can use an enum and store the enum with different types inside vector.

## Strings

- Strings are stored as a collection of UTF-8 encoded bytes.
- We can add additional strings to an existing string using `push_str` method and add a single character using `push` method.
- We can also concatenate two strings using the `format!` macro.
- The `format!` macro does not take the ownership of the strings passed in.
- In order to access the first letter in a string, we need to first convert the string to either `bytes` or `chars`.

## Hashmaps

- Hashmap is a `key` `value` store, the `key` and `value` can be of any type.
- To use hashmap, `use std::collection::Hashmap`.
- To create a new empty Hashmap, use `HashMap::new()`.
- To add values into the hashmap, we use the `insert` method.
- To fetch the value of a key, we use `.get(<keyname>)` method.
- We can use `for in` to iterate over the hashmap.

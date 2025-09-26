# learn_rust_lets_get_rusty

## [Video 11: Traits in Rust!](https://www.youtube.com/watch?v=T0Xfltu4h3A&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=11)

### This chapter covers about the following

- [Traits](#traits)
- [Trait Bounds](#trait-bounds)

## Traits

- Traits can be used to implement shared behavior or functionality.
- Traits could have only the method definitions.
- The structs which implements those traits will have the actual implementation of those methods.
- Traits can also have default implementations for its methods.
- Methods in traits can call other methods inside the trait.

## Trait Bounds

- Trait bounds are used to restrict the usage to specific traits rather than generic.
- We can also specify either one trait bound or multiple trait bounds using `+` operator.
- `where` clause is used to specify multiple trait bounds

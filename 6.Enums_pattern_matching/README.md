# learn_rust_lets_get_rusty

## [Video 6: Enums and Pattern Matching in Rust!](https://www.youtube.com/watch?v=DSZqIJhkNCM&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=6)

### This chapter covers about the following

- [Enums](#enums)
- [Option Enum](#option-enum)
- [Pattern Matching](#pattern-matching)

## Enums

- Enums allow us to enumerate a list of variants.
- Enum variants can store different variety of data.
- Similar to structs, Enums can also have `methods` and `associated functions` using a `impl` implementation block.

## Option Enum

- Option Enum has only two variants `Some(T)` or `None`.
- `Some` represents any value.
- `None` represents no value.
- We don't have to manually annotate the type for `Some` variants because it is inferred from the passed value.
- But for None variant, we need to annotate the type.

## Pattern Matching

- To match and validate the variants of an Enum, we use the `match` statement which has to match all the variants of an Enum.
- To match all other variants of an Enum, We can use `_` to cover the remaining variants.
- We can also use a `if let` syntax to match only the variant we need to check.

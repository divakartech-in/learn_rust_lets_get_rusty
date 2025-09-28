# learn_rust_lets_get_rusty

## [Video 15: Writing a CLI App in Rust - Part 1!](https://www.youtube.com/watch?v=XYkiwsplDTg&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=15)

### This chapter covers about the following

- [Building a minigrep CLI app](#building-a-minigrep-cli-app)
- [Parsing user input](#parsing-user-input)
- [Reading file content](#reading-file-content)

## Building a minigrep CLI app

- We will build a minigrep CLI app which does the following:
  - Read the user input search query.
  - Read the filename input.
  - Read the file content based on filename.
  - Search if the query term is present in user input file content.

## Parsing user input

- To parse the user input query and filename, we need to import `use std::env`.
- `env::args().collect()` will parse the args and convert it into a collection.
- We will create a `struct` to store the `query` and `filename`.

## Reading file content

- To read the file content, we need to import `use std::fs`.

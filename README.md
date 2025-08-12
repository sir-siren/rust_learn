# Learn to Code with Rust

![Github Download](assets/RustCrab.png)

Welcome! These are the course materials for the **Learn to Code with Rust** course.

## Course Content

This repository is structured into several Cargo projects, each focusing on a specific topic in Rust.

### Learning Modules

| #  | Topic                               | Directory                                                                            | Description                                                                                                                               |
|----|-------------------------------------|--------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------|
| 1  | Getting Started                     | [01-getting-started](./01-getting-started/)                                           | Introduces the basics of Rust, including `cargo`, the Rust compiler, and the `println!` macro.                                            |
| 2  | Variables & Mutability              | [02-variables_&_mutability](./02-variables_&_mutability/)                               | Covers variables, mutability, constants, and shadowing.                                                                                   |
| 3  | Data Types                          | [03-data_types](./03-data_types/)                                                       | Explores scalar and compound data types, including integers, floats, booleans, characters, tuples, and arrays.                            |
| 4  | Functions                           | [04-functions](./04-functions/)                                                         | Teaches how to define and use functions in Rust.                                                                                          |
| 5  | Control Flow                        | [05-control-flow](./05-control-flow/)                                                   | Covers `if`, `else if`, `else`, `match`, `loop`, `while`, and `for` expressions.                                                            |
| 6  | Ownership                           | [06-ownership](./06-ownership/)                                                         | Introduces Rust's ownership system, which is one of its most unique and powerful features.                                                |
| 7  | References & Borrowing              | [07-references_borrowing](./07-references_borrowing/)                                   | Explains how to use references to borrow access to data without taking ownership.                                                         |
| 8  | Slices                              | [08-slices](./08-slices/)                                                               | Covers slices, which provide a way to reference a contiguous sequence of elements in a collection rather than the whole collection.         |
| 9  | Structs                             | [09-structs](./09-structs/)                                                             | Teaches how to create custom data types using structs.                                                                                    |
| 10 | Enums                               | [10-enums](./10-enums/)                                                               | Explores enumerations, which allow you to define a type by enumerating its possible variants.                                             |
| 11 | Generics                            | [11-generics](./11-generics/)                                                         | Covers generics, which allow you to write code that can work with multiple data types.                                                    |
| 12 | Option and Result Enums             | [12-option-and-result-enums](./12-option-and-result-enums/)                           | Introduces the `Option` and `Result` enums, which are used for handling the absence of a value and for error handling, respectively.      |
| 13 | Vectors                             | [13-vectors](./13-vectors/)                                                           | Teaches how to use vectors, which are resizable arrays.                                                                                   |
| 14 | Project Structure                   | [14-project-structure](./14-project-structure/)                                         | Explains how to structure a Rust project with modules and multiple files.                                                                 |
| 15 | Strings                             | [15-strings](./15-strings/)                                                           | Covers the `String` and `&str` types, and how to work with text in Rust.                                                                  |
| 16 | Hash Maps                           | [16-hash-maps](./16-hash-maps/)                                                       | Teaches how to use hash maps, which are collections of key-value pairs.                                                                   |
| 17 | Error Handling                      | [17-error-handling](./17-error-handling/)                                             | Covers more advanced error handling techniques, including the `?` operator and the `Error` trait.                                         |
| 18 | Traits                              | [18-traits](./18-traits/)                                                             | Introduces traits, which are a way to define shared behavior across different types.                                                      |
| 19 | Lifetimes                           | [19-lifetimes](./19-lifetimes/)                                                       | Explains lifetimes, which are a way to ensure that references are valid for as long as they need to be.                                   |
| 20 | Closures                            | [20-closures](./20-closures/)                                                         | Covers closures, which are anonymous functions that can capture their environment.                                                        |
| 21 | Iterators                           | [21-iterators](./21-iterators/)                                                       | Teaches how to use iterators, which provide a way to process a sequence of items.                                                         |
| 22 | Testing                             | [22-testing](./22-testing/)                                                           | Explains how to write tests in Rust, including unit tests, integration tests, and documentation tests.                                    |
| 23 | Random                              | [23-random](./23-random/)                                                             | Covers how to generate random numbers in Rust using the `rand` crate.                                                                     |
| 24 | Datetimes                           | [24-datetimes](./24-datetimes/)                                                       | Teaches how to work with dates and times in Rust using the `chrono` crate.                                                                |
| 25 | Regular Expressions                 | [25-regular-expressions](./25-regular-expressions/)                                   | Covers how to use regular expressions in Rust using the `regex` crate.                                                                    |
| 26 | Smart Pointers (Box)                | [26-smart-pointers-box](./26-smart-pointers-box/)                                     | Introduces smart pointers, starting with `Box<T>`, which allows you to store data on the heap.                                            |

### Projects

| Project      | Directory                               | Description                                                                                                                               |
|--------------|-----------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------|
| Saladworks   | [saladworks](./saladworks/)             | A project for practicing the concepts learned in the course.                                                                              |
| Warehouse    | [warehouse](./warehouse/)               | A project for practicing the concepts learned in the course.                                                                              |

## Working Through the Course

The `learn-to-code-with-rust` folder contains multiple directories. Each folder is an independent Cargo project with a `Cargo.toml` file.

At the start of each course section, you'll find a note listing the Cargo project for that course.

For example, the next section is called "Variables and Mutability" and has a corresponding `variables-and-mutability` folder.

![Github Download](assets/SampleFolder.png)

Open up the section's folder in VSCode. There are two ways to accomplish this:

- In VSCode, access the main menu. Select `File > Open Folder...`. Locate the section's project folder on your file system and select it.

![Github Download](assets/OpenFolder.png)

- In Terminal/PowerShell, navigate into the section's folder with the `cd` command. Then, open the folder in VSCode with `code .`

```sh
$ cd variables-and-mutability

$ code .
```

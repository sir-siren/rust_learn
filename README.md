# Learn to Code with Rust

![Github Download](assets/RustCrab.png)

Welcome! These are the course materials for the **Learn to Code with Rust** course.

## Course Content

This repository is structured into several Cargo projects, each focusing on a specific topic in Rust.

### Learning Modules

| #  | Topic                               | Directory                                                                            |
|----|-------------------------------------|--------------------------------------------------------------------------------------|
| 1  | Getting Started                     | [1-getting-started](./1-getting-started/)                                           |
| 2  | Variables & Mutability              | [2-variables_&_mutability](./2-variables_&_mutability/)                               |
| 3  | Data Types                          | [3-data_types](./3-data_types/)                                                       |
| 4  | Functions                           | [4-functions](./4-functions/)                                                         |
| 5  | Control Flow                        | [5-control-flow](./5-control-flow/)                                                   |
| 6  | Ownership                           | [6-ownership](./6-ownership/)                                                         |
| 8  | Slices                              | [8-slices](./8-slices/)                                                               |
| 9  | Structs                             | [9-structs](./9-structs/)                                                             |
| 10 | Enums                               | [10-enums](./10-enums/)                                                               |
| 11 | Generics                            | [11-generics](./11-generics/)                                                         |
| 12 | Option and Result Enums             | [12-option-and-result-enums](./12-option-and-result-enums/)                           |
| 13 | Vectors                             | [13-vectors](./13-vectors/)                                                           |
| 15 | Strings                             | [15-strings](./15-strings/)                                                           |
| 16 | Hash Maps                           | [16-hash-maps](./16-hash-maps/)                                                       |
| 17 | Error Handling                      | [17-error-handling](./17-error-handling/)                                             |
| 18 | Traits                              | [18-traits](./18-traits/)                                                             |
| 19 | Lifetimes                           | [19-lifetimes](./19-lifetimes/)                                                       |
| 20 | Closures                            | [20-closures](./20-closures/)                                                         |
| 21 | Iterators                           | [21-iterators](./21-iterators/)                                                       |
| 22 | Testing                             | [22-testing](./22-testing/)                                                           |
| 23 | Random                              | [23-random](./23-random/)                                                             |
| 24 | Datetimes                           | [24-datetimes](./24-datetimes/)                                                       |
| 25 | Regular Expressions                 | [25-regular-expressions](./25-regular-expressions/)                                   |
| 26 | Smart Pointers (Box)                | [26-smart-pointers-box](./26-smart-pointers-box/)                                     |

### Projects

| Project      | Directory                               |
|--------------|-----------------------------------------|
| Saladworks   | [saladworks](./saladworks/)             |
| Warehouse    | [warehouse](./warehouse/)               |

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
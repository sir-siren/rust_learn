---
applyTo: '**'
---

# Rust Learning Repository - AI Agent Instructions

This repository contains a series of independent Rust projects, each designed to teach a specific concept. AI agents should be aware of the following conventions and workflows to be immediately productive in this codebase.

## Project Structure

Each numbered directory (e.g., `01-getting-started/`, `02-variables_&_mutability/`) represents a self-contained Rust project. These projects typically contain:
- `Cargo.toml`: Project manifest and dependency definitions.
- `src/`: Source code for the project, usually `main.rs` or `lib.rs`.
- `target/`: Compiled artifacts and build outputs.
- `assets/`: (Optional) Supplementary files like images or additional text.

There are also two larger projects, `saladworks/` and `warehouse/`, which serve as practice environments.

## Critical Developer Workflows

### Building and Running Projects

To build and run any individual project, navigate into its root directory (the one containing `Cargo.toml`) and use the standard Cargo commands:

- **Build:** `cargo build`
- **Run:** `cargo run`
- **Check (for errors without building):** `cargo check`
- **Run tests:** `cargo test`

Example for `01-getting-started/hello`:
```sh
cd 01-getting-started/hello
cargo run
```

### Dependencies

Dependencies for each project are declared in its respective `Cargo.toml` file. When adding new functionality that requires external crates, ensure they are added to the `[dependencies]` section of the relevant `Cargo.toml`.

### Rust-Specific Conventions

- **Ownership and Borrowing:** Rust's ownership system is fundamental. AI agents should prioritize understanding and adhering to ownership rules to prevent common Rust errors related to borrowing and lifetimes.
- **Error Handling:** Rust primarily uses the `Result<T, E>` enum for recoverable errors and `panic!` for unrecoverable errors. Agents should favor `Result` for robust error handling.
- **Testing:** Tests are typically located in `src/main.rs`, `src/lib.rs`, or in a `tests/` directory within the project root. Unit tests are often inline with the code they test, marked with `#[test]`.

## Integration Points and External Dependencies

While most modules are self-contained, some later modules introduce external crates (e.g., `rand` for random numbers, `chrono` for datetimes, `regex` for regular expressions). When working with these, refer to the `Cargo.toml` for the specific crate name and version, and consult the official crate documentation for usage.

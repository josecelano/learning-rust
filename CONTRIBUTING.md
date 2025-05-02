# Contributing to Rust Learning Journey

Thank you for your interest in contributing to this repository! While this is currently a private repository for personal learning, we welcome suggestions and improvements.

## Code of Conduct

This project and everyone participating in it is governed by the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## How to Contribute

1. **Fork the Repository**
   - Create your own fork of the project
   - Clone your fork locally

2. **Create a New Branch**
   - Create a branch for your changes
   - Use a descriptive name for your branch (e.g., `feature/add-lifetime-examples`)

3. **Make Your Changes**
   - Write clear, well-documented code
   - Include tests for your examples
   - Add or update documentation as needed
   - Follow Rust style guidelines

4. **Documentation**
   - Each example should include:
     - A README.md explaining the concept
     - Inline documentation
     - Examples of usage
     - Common pitfalls or gotchas

5. **Testing**
   - Ensure all tests pass: `cargo test`
   - Add new tests for new functionality
   - Consider edge cases and error conditions

6. **Commit Your Changes**
   - Write clear commit messages
   - Reference any relevant issues
   - Keep commits focused and atomic

## Code Style

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- Use `rustfmt` to format your code
- Run `clippy` to catch common mistakes
- Document public APIs with `///` comments

## Example Structure

Each example should follow this structure:

```text
example_name/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   └── lib.rs
└── tests/
    └── tests.rs
```

## Pull Request Process

1. Update the README.md with details of changes if needed
2. Update the documentation if you're changing functionality
3. The PR will be merged once you have the sign-off of at least one maintainer

## Questions?

Feel free to open an issue if you have any questions about contributing.

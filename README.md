# Learning Rust

This repository contains a collection of Rust examples, patterns, and concepts
that I've learned while working with Rust in a professional environment. Each
example is designed to be self-contained and includes detailed explanations.

> DISCLAIMER: I'm using <https://www.cursor.com/> and  Visual Studio Code with
Copilot (and others in the future) to generate the code, so some of the code
might not be 100% correct.

## 🎯 Purpose

The main goals of this repository are to:

- Document Rust concepts and patterns learned during daily development
- Provide practical, real-world examples of Rust features
- Create a personal knowledge base for future reference
- Share knowledge with the Rust community

## 📚 Use Cases

The repository contains the following examples:

1. **Newtype Pattern** - A pattern for creating type-safe wrappers around existing types
   - Basic type safety with UserId and EmailAddress
   - Configuration option flags for improved readability
2. **Enum Methods and Traits** - Demonstrating how enums can implement methods and traits
3. **Lifetime, Type, and Const Order** - Exploring the flexibility of parameter ordering in Rust
4. **Generic Impl for Generic Structs** - Understanding why `impl<T>` is needed for generic methods on generic structs
5. **Non-Primitive Constants** - Using complex types like structs as constants
6. **Implementing for Composite Types** - Implementing traits for types like `Arc<OtherType>`
7. **Generic Traits with Associated Types** - Comparing associated types vs. generic methods in traits

Each example is contained in its own directory with:

- A README.md explaining the concept
- Source code demonstrating the concept
- Tests to verify the behavior
- Comments explaining the implementation

## 🚀 Getting Started

1. Clone the repository:

   ```bash
   git clone https://github.com/josecelano/learning-rust.git
   cd learning-rust
   ```

2. Build all examples:

   ```bash
   cargo build
   ```

3. Run all examples:

   ```bash
   ./bin/run_examples.sh
   ```

4. Run tests:

   ```bash
   cargo test
   ```

## 🔄 Continuous Integration

This project uses GitHub Actions for continuous integration. The CI workflow:

- Runs on pull requests and pushes to the main branch
- Checks code formatting with `cargo fmt`
- Runs linting with `cargo clippy`
- Builds all examples
- Runs all tests
- Executes all examples

You can see the CI status on the [Actions tab](https://github.com/josecelano/learning-rust/actions) of the GitHub repository.

```bash
./bin/run_examples.sh
```

## 📝 Contributing

Contributions are welcome! This repository aims to be a valuable resource for the Rust community. Here's how you can contribute:

1. **Running Examples**

   - All examples can be run using the provided script:

   ```bash
   ./bin/run_examples.sh
   ```

   - Individual examples can be run using:

   ```bash
   cargo run -p <example_name>
   ```

   For example, to run the Enum Methods example:

   ```bash
   cargo run -p enum_methods
   ```

   If a package has multiple binaries, you need to specify which one:

   ```bash
   cargo run -p newtype_pattern --bin basic_newtype
   ```

   Available examples:

   - `cargo run -p enum_methods` - Shows shape calculations and trait implementations
   - `cargo run -p newtype_pattern --bin basic_newtype` - Demonstrates type-safe wrappers
   - `cargo run -p newtype_pattern --bin config_flag` - Shows configuration flags
   - `cargo run -p lifetime_order` - Demonstrates parameter ordering
   - `cargo run -p generic_impl` - Shows generic implementations
   - `cargo run -p non_primitive_constants` - Shows complex constants
   - `cargo run -p composite_impl` - Shows wrapper implementations
   - `cargo run -p generic_traits` - Shows trait implementations with associated types

2. **Adding New Examples**

   - Create a new directory in `src/` with a descriptive name
   - Include a `Cargo.toml` and `README.md`
   - Add your example code in `src/bin/`
   - Update the workspace members in the root `Cargo.toml`
   - Update this README's Use Cases section

3. **Improving Existing Examples**

   - Fix bugs or improve code clarity
   - Add more test cases
   - Enhance documentation
   - Add performance optimizations

4. **Submitting Changes**

   - Fork the repository
   - Create a feature branch
   - Make your changes
   - Run all examples to ensure everything works
   - Submit a pull request with a clear description

5. **Code Style**

   - Follow Rust standard formatting (`cargo fmt`)
   - Run clippy checks (`cargo clippy`)
   - Ensure all examples compile without warnings
   - Add comments explaining complex concepts

6. **Documentation**

   - Keep README files up to date
   - Add inline documentation for complex code
   - Include usage examples
   - Explain the "why" not just the "what"

For questions or discussions, please open an issue. Let's make this resource better together!

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The Rust community for their excellent documentation and resources.
- All the open-source projects that have inspired these examples.
- Eduardo Ferro Aldama for writing the wonderful serries of articles about Vive Coding (I've copied some of the content from his articles):
  - [Vibe coding: building things from curiosity and flow](https://www.eferro.net/2025/03/vibe-coding-building-things-from.html)
  - [Vibe coding II: when flow meets tests](https://www.eferro.net/2025/04/vibe-coding-ii-when-flow-meets-tests.html)
  - [Vibe Coding III: Complexity Creeps—Unless You Don’t Let It](https://www.eferro.net/2025/04/vibe-coding-iii-complexity-creepsunless.html)

## 🔗 Similar Projects

Here are some other excellent Rust learning resources:

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn Rust through examples
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises to get you used to reading and writing Rust code
- [Rust Design Patterns](https://github.com/rust-unofficial/patterns) - A catalogue of Rust design patterns, anti-patterns and idioms
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/) - A collection of simple examples that demonstrate good practices
- [Rust Quiz](https://dtolnay.github.io/rust-quiz/) - A collection of Rust quizzes to test your knowledge
- [Rust Playground](https://play.rust-lang.org/) - An online playground for Rust code
- [Rust Reference](https://doc.rust-lang.org/reference/) - The Rust Reference Manual
- [Rust Book](https://doc.rust-lang.org/book/) - The Rust Programming Language Book

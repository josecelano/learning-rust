# 04. Generic Impl for Generic Structs

This example demonstrates why `impl<T>` is needed for generic methods on generic structs, and how it differs from concrete implementations.

## Key Concepts

- Understanding the difference between `impl<T>` and `impl` for generic structs
- How type parameters are scoped in implementations
- When to use generic implementations vs. concrete implementations
- Common patterns for implementing methods on generic types

## Examples

The examples in this directory demonstrate:

1. Generic struct with generic methods
2. Generic struct with concrete methods
3. Concrete struct with generic methods
4. Concrete struct with concrete methods
5. Comparing different implementation approaches

## Running the Examples

```bash
cargo run
```

## Running the Tests

```bash
cargo test
```

## Common Patterns

- Using `impl<T>` when the method itself is generic
- Using `impl` when the method is specific to the struct's type parameter
- Implementing traits for generic structs

## Further Reading

- [The Rust Book - Advanced Traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)
- [The Rust Reference - Implementations](https://doc.rust-lang.org/reference/items/implementations.html)

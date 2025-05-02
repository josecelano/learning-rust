# 07. Generic Traits with Associated Types

This example demonstrates the difference between generic traits with associated types and generic methods, and when to use each approach in Rust.

## Key Concepts

- Defining traits with associated types
- Defining traits with generic methods
- Implementing traits for generic structs
- Type safety and trait flexibility
- When to use associated types vs. generic methods

## Examples

The examples in this directory demonstrate:

1. A trait with an associated type (`Container`)
2. A trait with generic methods (`Storage<T>`)
3. Implementing traits for generic structs
4. Type safety differences between the two approaches

## Running the Examples

```bash
cargo run
```

## Running the Tests

```bash
cargo test
```

## Common Use Cases

- Designing flexible and reusable APIs
- Enforcing type relationships in trait implementations
- Choosing between associated types and generic methods for trait design
- Building abstractions over containers and storage types

## Further Reading

- [The Rust Book - Advanced Traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)
- [The Rust Reference - Traits](https://doc.rust-lang.org/reference/items/traits.html)
- [Rust by Example - Associated Types](https://doc.rust-lang.org/rust-by-example/trait/associated_types.html)

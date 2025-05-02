# 01. Newtype Pattern

The newtype pattern is a Rust idiom that involves creating a new type that wraps another type. This pattern provides type safety and allows you to implement traits for the new type without modifying the original type.

## Key Concepts

- Creating type-safe wrappers around existing types
- Implementing traits for the new type
- Zero-cost abstraction (no runtime overhead)
- Preventing mixing of different types that have the same underlying representation

## Examples

The examples in this directory demonstrate:

1. Basic newtype pattern with a simple wrapper
2. Implementing traits for the newtype
3. Using the newtype pattern for type safety
4. Converting between the newtype and the underlying type
5. Using newtype pattern for configuration option flags

## Running the Examples

```bash
cargo run
```

To run a specific example:

```bash
cargo run --bin basic_newtype
cargo run --bin config_flag
```

## Running the Tests

```bash
cargo test
```

## Common Use Cases

- Creating domain-specific types (e.g., `UserId`, `EmailAddress`)
- Implementing traits for types from external crates
- Preventing mixing of different units (e.g., `Meters` vs `Feet`)
- Adding validation or constraints to existing types

## Further Reading

- [The Rust Book - Advanced Types](https://doc.rust-lang.org/book/ch19-04-advanced-types.html)
- [Rust by Example - Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)

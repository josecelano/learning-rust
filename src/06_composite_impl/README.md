# 06. Implementing for Composite Types

Rust allows you to implement traits for composite types like `Arc<T>`, `Box<T>`, and other wrapper types, not just for concrete named types.

## Key Concepts

- Implementing traits for wrapper types
- Using blanket implementations
- Implementing traits for type aliases
- Understanding orphan rules in this context

## Examples

The examples in this directory demonstrate:

1. Implementing traits for `Arc<T>`
2. Implementing traits for `Box<T>`
3. Implementing traits for custom wrapper types
4. Using blanket implementations with composite types

## Running the Examples

```bash
cargo run
```

## Running the Tests

```bash
cargo test
```

## Common Use Cases

- Adding functionality to types from external crates
- Creating ergonomic APIs for wrapper types
- Implementing traits for type aliases
- Extending functionality of standard library types

## Further Reading

- [The Rust Book - Advanced Traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)
- [The Rust Reference - Orphan Rules](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules)

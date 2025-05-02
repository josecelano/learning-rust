#!/bin/bash

# Print a header for each example
print_header() {
    echo -e "\n\033[1;34m=== Running Example $1 ===\033[0m\n"
}

# Run each example
print_header "01: Newtype Pattern"
cargo run -p newtype_pattern --bin basic_newtype

print_header "02: Enum Methods"
cargo run -p enum_methods

print_header "03: Lifetime Order"
cargo run -p lifetime_order

print_header "04: Generic Impl"
cargo run -p generic_impl

print_header "05: Non-Primitive Constants"
cargo run -p non_primitive_constants

print_header "06: Composite Impl"
cargo run -p composite_impl

print_header "07: Generic Traits"
cargo run -p generic_traits

echo -e "\n\033[1;32mAll examples completed!\033[0m"

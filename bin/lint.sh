#!/bin/bash

CARGO_INCREMENTAL=0 cargo clippy \
    --no-deps --tests --benches --examples --workspace --all-targets --all-features -- \
    -D clippy::correctness \
    -D clippy::suspicious \
    -D clippy::complexity \
    -D clippy::perf \
    -D clippy::style \
    -D clippy::pedantic


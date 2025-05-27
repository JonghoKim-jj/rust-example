# Rust-example

A simple example/test for Rust, GitHub issue, and PR.

## Before you

To pass CI tests, you should run local checks.

Check format and lint:

```sh
cargo fmt -- --check --config group_imports=StdExternalCrate
cargo clippy --tests -- -D warnings -W clippy::pedantic
```

Check build and tests:

```sh
cargo build --examples --verbose
cargo test --verbose
```

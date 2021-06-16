# Project tooling

This package contains helper utilities for this project. They should remain
cross-platform, and be written in Rust, so as to minimize project dependencies.

## `runner`

When you say `cargo run` in this workspace, Cargo will build and run this crate
after building your Teensy 4 program. This crate will

- convert your program from ELF to IHEX using `rust-objcopy`
- invoke `teensy_loader_cli`, and program your board

Try it out:

```
cargo run --example led --target thumbv7em-none-eabihf --release --features rt
```

Requires all build dependencies, including `teensy_loader_cli`. See the project
[README](../README.md#dependencies) for more information.

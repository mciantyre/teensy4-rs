# `teensy4-bsp` Examples

This directory contains examples that run on your Teensy 4.0 or Teensy 4.1.
We separate examples based on their dependencies:

- The examples prefixed with `rtic_*` demonstrate the [RTIC] framework
- Otherwise, the examples demonstrate how to directly use the BSP

[RTIC]: https://rtic.rs/0.5/book/en/

## Build and run examples

Make sure you have all of the build dependencies described in the [top-level
README](../README.md#dependencies).

First, build all of the examples for the MCU. Enable all BSP features to build
all examples.

```
cargo build --release --examples --all-features --target thumbv7em-none-eabihf
```

Convert your example of interest to a HEX file. For instance, to convert the
`led` example, run

```
rust-objcopy -O ihex target/thumbv7em-none-eabihf/release/examples/led led.hex
```

Finally, load the HEX file onto your board. 

To understand what each example should do, see the example's documentation at
the top of the file.

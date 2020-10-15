# `teensy4-bsp` Examples

This directory contains examples that run on your Teensy 4.0 or Teensy 4.1.
We separate examples based on their dependencies:

- The `bsp` directory contains examples that simply use the `teensy4-bsp`
  crate.
- The `rtic` directory contains examples that combine the BSP with the [RTIC]
  embedded Rust framework.

[RTIC]: https://rtic.rs/0.5/book/en/

## Build and run BSP examples

Make sure you have all of the build dependencies described in the [top-level
README](../README.md#dependencies).

To build a BSP example, enter the `bsp` directory, and use `cargo objcopy` to
create a hex program.

```
cd examples/bsp
cargo objcopy --release --bin led -- -O ihex led.hex
```

You should find an `led.hex` output. Download the file to your Teensy, and
watch the LED turn on!

To select a different example, change the argument to `--bin`.

## Build and run RTIC examples

Follow the same process for building a BSP example, but use the examples in the
`rtic` directory:

```
cd examples/rtic
cargo objcopy --release --bin rtic_blink -- -O ihex rtic_blink.hex
```

## Runtime support

The `Cargo.toml` files in each example directory shows how you might depend on
the `teensy4-rt` in your applications. See those files for more information.
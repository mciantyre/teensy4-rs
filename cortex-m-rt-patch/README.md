# `cortex-m-rt` patch for teensy4

The sole purpose of this crate is to allow for patching the `cortex-m-rt`
dependency of upstream crates with the `teensy4-rt` crate. To understand why
this is necessary, please refer to [the Runtime section][1] of the top-level
README.

If you are looking for the repository of the real `cortex-m-rt` crate, see
[here][2].

## Usage

Add the `[patch]` for `cortex-m-rt` to your project like so:

```toml
# Patch `cortex-m-rt` for reasons described here:
# https://github.com/mciantyre/teensy4-rs#runtime
[patch.crates-io.cortex-m-rt]
git = "https://github.com/mciantyre/teensy4-rs"
branch = "master"
```

[1]: https://github.com/mciantyre/teensy4-rs#runtime
[2]: https://github.com/rust-embedded/cortex-m-rt

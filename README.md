# teensy4-rs

A collection of crates for Rust development on the Teensy 4. Supports
the following boards:

-   Teensy 4.0
-   Teensy 4.1
-   Teensy MicroMod

[![Code Checks][]][1] [![crates.io][]][2] [![docs.rs]][3]

  [Code Checks]: https://github.com/mciantyre/teensy4-rs/workflows/Code%20Checks/badge.svg
  [1]: https://github.com/mciantyre/teensy4-rs/actions?query=workflow%3A%22Code+Checks%22
  [crates.io]: https://img.shields.io/crates/v/teensy4-bsp
  [2]: https://crates.io/crates/teensy4-bsp
  [docs.rs]: https://docs.rs/teensy4-bsp/badge.svg
  [3]: https://docs.rs/teensy4-bsp/

#### [API Docs (`master`)]

  [API Docs (`master`)]: https://mciantyre.github.io/teensy4-rs/

## Dependencies

-   A Rust installation. Install Rust using `rustup`. We support the
    latest, stable Rust toolchain.

-   The `thumbv7em-none-eabihf` Rust target, which may be installed
    using `rustup`:

        rustup target add thumbv7em-none-eabihf

-   A capable `objcopy` for transforming Rust binaries into hex files.
    The documentation and tooling in the project uses the LLVM `objcopy`
    provided by [`cargo-binutils`]. Install [`cargo-binutils`] if you
    want to precisely follow this documentation.

-   To download programs to your Teensy 4, you'll need either a build of
    [`teensy_loader_cli`], or the [Teensy Loader Application]. The
    latter is available with the Teensyduino add-ons.

  [`cargo-binutils`]: https://github.com/rust-embedded/cargo-binutils
  [`teensy_loader_cli`]: https://github.com/PaulStoffregen/teensy_loader_cli
  [Teensy Loader Application]: https://www.pjrc.com/teensy/loader.html

## Getting started

Use our `cargo-generate` template, [`teensy4-rs-template`], to bootstrap
your own `teensy4-rs` project based on these libraries:

    cargo install cargo-generate
    cargo generate --git https://github.com/mciantyre/teensy4-rs-template --name hello-world
    cd hello-world
    cargo objcopy --release -- -O ihex hello-world.hex

Download `hello-world.hex` to your Teensy 4!

See the Rust documentation for API information. In particular, study the
[`imxrt-hal` APIs], since the BSP forwards many of the HAL's interfaces:

    cargo doc --open

Try the various examples in this project's [`examples` directory] if
you'd like to test your system.

  [`teensy4-rs-template`]: https://github.com/mciantyre/teensy4-rs-template
  [`imxrt-hal` APIs]: https://docs.rs/imxrt-hal/latest/imxrt_hal/
  [`examples` directory]: examples/README.md

## Contributing

We welcome support! A great way to contribute is to start using the
crates to develop Teensy 4 applications. Submit an issue to help us
identify bugs, feature requests, or documentation gaps. See
[CONTRIBUTING.md] to learn about the best issue tracker for your
request.

If you want to directly contribute to the `teensy4-rs` project, read the
development guidance in [CONTRIBUTING.md].

  [CONTRIBUTING.md]: CONTRIBUTING.md

## Acknowledgements and References

-   The [Teensy 4] is wonderful, and that's thanks to the hard work of
    PJRC and friends. We can find the Teensy code used in the Arduino
    plugins [here]. The code greatly influenced this library.
-   The Rust Cortex M team, specifically the [`cortex-m-rt`] crate.

  [Teensy 4]: https://www.pjrc.com/store/teensy40.html
  [here]: https://github.com/PaulStoffregen/cores
  [`cortex-m-rt`]: https://github.com/rust-embedded/cortex-m-rt

## License

Licensed under either of

-   Apache License, Version 2.0 ([LICENSE-APACHE] or
    http://www.apache.org/licenses/LICENSE-2.0)
-   MIT license ([LICENSE-MIT] or http://opensource.org/licenses/MIT)

at your option.

  [LICENSE-APACHE]: LICENSE-APACHE
  [LICENSE-MIT]: LICENSE-MIT

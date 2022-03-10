# teensy4-rs

A collection of crates for Rust development on the Teensy 4. Supports
both the Teensy 4.0 and 4.1 boards.

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

## Project Structure

The project has a model similar to other embedded Rust projects. We have
a custom runtime crate to support our processor and memory layout. We
use a separate iMXRT register access layer (RAL) and hardware
abstraction layer (HAL). The RAL and HAL are provided by the [`imxrt-rs`
project].

The main crate is a board support package (BSP), `teensy4-bsp`, for the
Teensy 4. The BSP lets you use the Teensy 4's pins and peripherals. It
also provides an implementation of the [`log` crate], allowing users to
log over USB. If you would like to develop Rust applications for the
Teensy 4, start by depending on the `teensy4-bsp`.

The BSP depends on the these additional crates, which are part of the
BSP's workspace:

-   `teensy4-fcb`: an FCB specific to the Teensy 4. It auto-generates
    the FCB using the [`imxrt-boot-gen`] crate.
-   `teensy4-pins`: a helper library to convert the processor's pads
    into the pins available on a Teensy 4.0 or 4.1 board.

See the API docs for information on runtime support and BSP features.

  [`imxrt-rs` project]: https://github.com/imxrt-rs
  [`log` crate]: https://crates.io/crates/log
  [`imxrt-boot-gen`]: https://github.com/imxrt-rs/imxrt-boot-gen

## Contributing

We welcome support! A great way to contribute is to start using the
crates to develop Teensy 4 applications. Submit an issue to help us
identify bugs, feature requests, or documentation gaps. See
[CONTRIBUTING.md] to learn about the best issue tracker for your
request.

If you want to directly contribute to the `teensy4-rs` project, read the
development guidance in [CONTRIBUTING.md].

  [CONTRIBUTING.md]: CONTRIBUTING.md

## Q/A

#### There's more C than Rust! How is this a Rust project?

There used to be more Rust code here. But today, most Rust development
happens in the [`imxrt-rs` project].

We have C sources in this project because

-   we can't easily express the equivalent Rust code on a stable
    compiler (runtime support)
-   we haven't written a Rust implementation to replace it (USB support)

We precompile these C sources so that our users do not need an ARM
toolchain to compile the crates.

  [`imxrt-rs` project]: https://github.com/imxrt-rs

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

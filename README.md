# teensy4-rs

A collection of crates for Rust development on the Teensy 4.

Status: prototype.

We can
- blink the LED
- statically register exceptions
- statically register interrupts
- log over USB
- read serial data over USB
- measure intervals with periodic interrupt timers
- talk to I2C slave devices
- control PWM outputs (single pin)
- accept serial (UART) data (although the hardware receiver buffer is small)
- transmit serial (UART) data
- send and receive data over SPI peripherals
- use DMA channels

We've measured a few things things, like I2C, UART, SPI, and timer timings. No one has built a fully-fledged application with these crates, yet...

![Code Checks](https://github.com/mciantyre/teensy4-rs/workflows/Code%20Checks/badge.svg)

## Dependencies

- A Rust installation; recommended installation via `rustup`.
- The `thumbv7em-none-eabihf` Rust target, which may be installed via `rustup`:

    ```bash
    $ rustup target add thumbv7em-none-eabihf
    ```

- A capable `objcopy` for transforming Rust binaries into hex files. The documentation and tooling in the project uses the LLVM `objcopy` provided by [`cargo binutils`](https://github.com/rust-embedded/cargo-binutils).

- Either a build of [`teensy_loader_cli`](https://github.com/PaulStoffregen/teensy_loader_cli), or the [Teensy Loader Application](https://www.pjrc.com/teensy/loader.html). The latter is available with the Teensyduino add-ons.

## Getting started

Use our `cargo-generate` template, [`teensy4-rs-template`](https://github.com/mciantyre/teensy4-rs-template), to bootstrap your own teensy4-rs project based on these libraries:

```
cargo install cargo-generate
cargo generate --git https://github.com/mciantyre/teensy4-rs-template --name hello-world
cd hello-world
cargo objcopy --release -- -O ihex hello-world.hex
```

Download `hellow-world.hex` to your Teensy 4!

See the Rust documentation for API information. In particular, study the `imxrt-hal` APIs, since the BSP forwards many of the HAL's interfaces:

```
cargo doc --open
```

Try the various examples in this project's `examples` directory if you'd like to test your system:

```
cargo objcopy --release --example led -- -O ihex led.hex
```

## Project Structure

The project has a model similar to other embedded Rust projects. We have a custom runtime crate to support our processor and memory layout. We use a separate iMXRT register access layer (RAL) and hardware abstraction layer (HAL). The RAL and HAL are provided by the [`imxrt-rs` project](https://github.com/imxrt-rs/imxrt-rs).

The main crate is a board support package (BSP) for the Teensy 4, called `teensy4-bsp`. The BSP provides access to the Teensy 4's pins and peripherals. It also provides an implementation of the [`log` crate](https://crates.io/crates/log), allowing users to log messages over USB. If you would like to develop Rust applications for the Teensy 4, start by depending on the `teensy4-bsp`.

The BSP relies on the following additional crates, which are part of the BSP's workspace:

- `teensy4-rt`: an API-compatible fork of the `cortex-m-rt` crate that describes the system's memory layout, startup sequence, and interrupt table. The runtime crate let's a user write a normal `main()` function. Unlike the `cortex-m-rt`, which tries to be a general runtime crate, the `teensy4-rt` crate is specific to the Teensy 4. See the "Runtime" notes to learn why this is a fork of the `cortex-m-rt` crate.
- `teensy4-fcb`: an FCB specific to the Teensy 4. It auto-generates the FCB using the [`imxrt-boot-gen`](https://github.com/imxrt-rs/imxrt-boot-gen) crate.
- `teensy4-usb-sys`: bindings to the Teensy 4's USB stack, which is written in C.

Although we strive for compatibility with existing crates and frameworks, we've introduced some custom modules in order to operate with the Teensy 4.0. We describe these differences below.

### Runtime

An embedded Rust developer might use the [`cortex-m-rt`](https://crates.io/crates/cortex-m-rt) to bootstrap a Cortex-M system. However, [#164](https://github.com/rust-embedded/cortex-m-rt/issues/164) notes that the `cortex-m-rt` crate cannot yet support devices with custom memory layouts. The iMXRT106x is one of the systems with a custom memory layout; in particular, we have tightly-coupled memory (TCM) regions for instructions (ITCM) and data (DTCM). We also need to place special arrays (the FCB) in memory in order to properly boot. Given these requirements, we need a custom runtime crate that can initialize the system.

The `teensy4-rt` crate is a fork of the `cortex-m-rt` crate that is customized to support a minimal iMXRT1062 startup and runtime. Like the `cortex-m-rt` crate, the `teensy3-rt` crate

- populates the vector table for correct booting and exception / interrupt dispatch
- initializes static variables
- enables the FPU (since we're a `thumbv7em-none-eabihf` device)

The `teensy4-rt` crate goes a step further in its startup functionality:

- provides the required firmware configuration block (FCB) placement and image vector table (IVT) in order to start the iMXRT106x
- initialize the TCM memory regions
- configures instruction and data caches based on the TCM regions

Just as the `cortex-m-rt` crate will call a user's `main()` function, the `teensy4-rt` completes by calling a user's `main()`. The `teensy4-rt` crate also exposes the `#[interrupt]`, `#[exception]`, and `#[entry]` macros for decorating interrupt handlers, exception handlers, and the program entrypoint, respectively. Note that, as of this writing, `#[pre_init]` is not supported.

To support compatibility with the `cortex-m-rt` crate, the `teensy4-rt` crate uses the same link sections as the `cortex-m-rt` crate. However, the `teensy4-rt` crate may locate memory in different regions. Specifically, all instructions are placed into ITCM, and all data is placed into DTCM.

It is our hope that the `teensy4-rt` crate can be transparently replaced with the `cortext-m-rt` crate once the necessary features are available. If you think that the `teensy4-rt` crate is be diverging from the `cortex-m-rt` crate and might miss that goal, please file an issue!

## Contributing

We welcome support! There are known issues that anyone can address in the issues tracker. And, the best way to contribute is to start using the crates to develop Teensy 4 applications. Submit an issue to help us identify bugs, feature requests, or documentation gaps. See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidance, and to learn about the best issue tracker for your request.

## Q/A

#### When will this be on crates.io?

After we evaluate whether or not this is a good or bad approach to developing Rust applications for the Teensy 4, we will either release these crates to crates.io, or recommend an alternative solution.

We also need to wait for all of our dependencies to become available on crates.io. As of this writing, we're waiting on an unreleased `cortex-m-rt-macros` crate. Until we release to crates.io, either

- use a git dependency (the default behavior of the [`teensy4-rs-template`](https://github.com/mciantyre/teensy4-rs-template))

    ```toml
    [dependencies.teensy4-bsp]
    git = "https://github.com/mciantyre/teensy4-rs.git"
    ```

- clone this repository, and specify the path to the dependency:

    ```toml
    [dependencies.teensy4-bsp]
    path = "path/to/cloned/teensy4-rs/teensy4-bsp"
    ```

## Acknowledgements and References

- The [Teensy 4](https://www.pjrc.com/store/teensy40.html) is wonderful, and that's thanks to the hard work of PJRC and friends. We can find the Teensy code used in the Arduino plugins [here](https://github.com/PaulStoffregen/cores). The code greatly influenced this library.
- The Rust Cortex M team, specifically the [`cortex-m-rt`](https://github.com/rust-embedded/cortex-m-rt) crate.


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
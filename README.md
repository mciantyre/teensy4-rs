# teensy4-rs

A collection of crates that support the development of Rust applications and libraries for the Teensy 4.

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

We've measured a few things things, like I2C, UART, SPI, and timer timings. No one has built a fully-fledged application with these crates, yet...

![Code Checks](https://github.com/mciantyre/teensy4-rs/workflows/Code%20Checks/badge.svg)

## Dependencies

- A Rust installation. We use the latest, stable Rust compiler. Minimum-Supported Rust Version (MSRV) is 1.40. Recommended installation via `rustup`.
- The `thumbv7-none-eabihf` Rust target, which may be installed via `rustup`:

```bash
$ rustup target add thumbv7em-none-eabihf
```

- The [GNU ARM Embedded Toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm) for compiling the C sources we need to bootstrap startup (in `teensy4-rt`) and the USB stack in the BSP. Specifically, we need the C compiler and archiver available on our path to build the runtime crate. For deploying to a Teensy 4 using the command-line loader, we'll also need `arm-none-eabi-objcopy` (ARM binutils).

- Optionally, a build of [`teensy_loader_cli`](https://github.com/PaulStoffregen/teensy_loader_cli) available on our path. We have a script to rapidly test example programs in the `teensy4-examples`, and it makes use of `teensy_loader_cli`. To load applications onto the Teensy 4, we may also use the [Teensy Loader Application](https://www.pjrc.com/teensy/loader.html), which is also available with the Teensyduino add-ons.

## Getting started

The best way to test your setup is to use the `hardware-test.sh` script (`hardware-test.bat` for Windows) to compile one of the examples. With a Teensy 4 connected to your system, build and load an example:

```
./hardware-test.sh led
```

If all goes well, the `led` example should turn on the Teensy 4's LED.

Use our `cargo-generate` template, [`teensy4-rs-template`](https://github.com/mciantyre/teensy4-rs-template), to bootstrap your own teensy4-rs project based on these libraries:

```
cargo install cargo-generate
cargo generate --git https://github.com/mciantyre/teensy4-rs-template --name hello-world
```

The `cargo-generate` template is great for quickly starting a project. But, if you'd like to manually set up a project, check out the getting started guide [here](docs/2020-01-03-getting-started.md).

As of this writing:

- both the `teensy4-rt`, and `teensy4-fcb` crates are necessary to successfully link a Teensy 4 Rust application.
- not all of the crates are published to crates.io, so we must either clone the repo and reference them locally, or reference the two crates via the git repository.

These crates are guaranteed to build when targeting `thumbv7em-none-eabihf`; we do not support any other targets.

To build the project in a Docker container, use our custom Docker image, available in the `docker` directory:

```
$ cd docker
$ docker build -t rust_teensy . 
```

Then, run the Docker image to build examples. In the snippet below, we build the `led` example: 

```
$ docker run -it --rm -v $PWD:/build rust_teensy led
```

## Project Structure

The project has a model similar to other embedded Rust projects. We have a custom runtime crate to support our processor and memory layout. We use a separate iMXRT register access layer (RAL) and hardware abstraction layer (HAL). The RAL and HAL are provided by the [`imxrt-rs` project](https://github.com/imxrt-rs/imxrt-rs). We add a board support package (BSP) for the Teensy 4 in this repository. The list below describes the project layout:

- `teensy4-rt`: an API-compatible fork of the `cortex-m-rt` crate that describes the system's memory layout, startup sequence, and interrupt table. The runtime crate let's a user write a normal `main()` function. Unlike the `cortex-m-rt`, which tries to be a general runtime crate, the `teensy4-rt` crate is specific to the Teensy 4. See the "Runtime" notes to learn why this is a fork of the `cortex-m-rt` crate.
- `teensy4-bsp`: a board support package (BSP) for the Teensy 4. The BSP provides access to the Teensy 4's pins and peripherals. It also provides an implementation of the [`log` crate](https://crates.io/crates/log), allowing users to log messages over USB. If you would like to develop Rust applications for the Teensy 4, start here.
- `teensy4-examples`: a collection of examples which run out-of-the-box on the Teensy 4. Take a look at the examples if you're interested in using these crates.
- `teensy4-fcb`: an FCB specific to the Teensy 4. It auto-generates the FCB using the [`imxrt-boot-gen`](https://github.com/imxrt-rs/imxrt-boot-gen) crate.

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

*When will this be on crates.io?*

After we evaluate whether or not this is a good or bad approach to developing Rust applications for the Teensy 4, we will either release these crates to crates.io, or recommend an alternative solution.

## Acknowledgements and References

- The [Teensy 4](https://www.pjrc.com/store/teensy40.html) is wonderful, and that's thanks to the hard work of PJRC and friends. We can find the Teensy code used in the Arduino plugins [here](https://github.com/PaulStoffregen/cores). The code greatly influenced this library.
- I'm not the only developer tackling the "Rust on Teensy 4" challenge. Check out mpasternacki's work [here](https://gitlab.com/teensy-rs/teensy-4) as an alternative approach towards the same problem.
- The Rust Cortex M team, specifically the [`cortex-m-rt`](https://github.com/rust-embedded/cortex-m-rt) crate.


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
# teensy4-rs

A collection of crates that support the development of Rust applications and libraries for the Teensy 4.

Status: prototype. We can blink the LED, register exceptions, register interrupts, and log over USB. No one has measured anything. No one has built a fully-fledged application with these crates, yet...

[![Build Status](https://travis-ci.org/mciantyre/teensy4-rs.svg?branch=master)](https://travis-ci.org/mciantyre/teensy4-rs)

## Dependencies

- A Rust installation. We use the latest, stable Rust compiler. Last tested on Rust 1.39.0. Recommended installation via `rustup`.
- The `thumbv7-none-eabihf` Rust target, which may be installed via `rustup`:

```bash
$ rustup target add thumbv7em-none-eabihf
```

- The [GNU ARM Embedded Toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm) for compiling the C sources we need to bootstrap startup (in `imxrt1060-rt`) and the USB stack in the BSP. Specifically, we need the C compiler and archiver available on our path to build the runtime crate. For deploying to a Teensy 4 using the command-line loader, we'll also need `arm-none-eabi-objcopy` (ARM binutils).

- Optionally, a build of [`teensy_loader_cli`](https://github.com/PaulStoffregen/teensy_loader_cli) available on our path. We have a script to rapidly test example programs in the `teensy4-examples`, and it makes use of `teensy_loader_cli`. To load applications onto the Teensy 4, we may also use the [Teensy Loader Application](https://www.pjrc.com/teensy/loader.html), which is also available with the Teensyduino add-ons.

## Getting started

The best way to test our setup is to use the `hardware-test.sh` script to compile one of the example binaries. With a Teensy 4 connected to our system, build and load an example:

```
./hardware-test.sh led
```

If all goes well, the `led` example should turn on the Teensy 4's LED.

To build new applications with these crates, include the `teensy4-bsp` crate. The BSP crate exposes the runtime, HAL, and the peripheral access crates:

```
[dependencies]
teensy4-bsp = { path = "path/to/teensy4-bsp" }
```

See the `teensy4-examples` crate for an example of a project that works on the Teensy 4.

Note that, as of this writing:

- both the `imxrt1060-pac`, `imxrt1060-rt`, and `teensy4-fcb` crates are necessary to successfully link a Teensy 4 Rust application. This requirement should be lifted in the future. See the discussion in the project structure section (below).
- the crates are not yet published to crates.io, so we must either clone the repo and reference them locally, or reference the two crates via the git repository.

These crates are guaranteed to build when targeting `thumbv7em-none-eabihf`; we do not support any other targets.

To build the project in a Docker comtainer, use our custom Docker image available in the `docker` directory:

```
$ cd docker
$ docker build -t rust_teensy . 
```

Then, run the Docker image to build examples. In the snippet below, we build the `led` example: 

```
$ docker run -it --rm -v $PWD:/build rust_teensy led
```

## Project Structure

The project has a model similar to other embedded Rust projects: we have a runtime crate, a peripheral access crate (PAC), and a board support package (BSP) for the Teensy 4. We also have a few crates that are unique to our system. The list below describes the project layout:

- `imxrt1060-fcb-gen`: a Rust crate used in other crates' build scripts. It provides an API for generating a Firmware Configuration Block (FCB), a memory region required to boot iMXRT106x processors. Other crates may use this crate to define custom FCBs for their iMXRT106x-based systems.
- `imxrt1060-hal`: a hardware abstraction layer (HAL) for the iMXRT106x. It provides implementations of the [`embedded-hal` traits](https://crates.io/crates/embedded-hal) for the processor's peripherals.
- `imxrt1060-pac`: a collection of Peripheral Access Crates (PAC), providing read / write APIs for processor registers. The crates are generated from [`svd2rust`](https://docs.rs/svd2rust/0.16.1/svd2rust/), with some custom tooling to generate a unique crate for each peripheral. See the "Peripheral access crates" notes to learn how this PAC might be different from other PACs.
- `imxrt1060-rt`: an API-compatible fork of the `cortex-m-rt` crate that describes the system's memory layout, startup sequence, and interrupt table. The runtime crate let's a user write a normal `main()` function. See the "Runtime" notes to learn why this is a fork of the `cortex-m-rt` crate.
- `teensy4-bsp`: a board support package (BSP) for the Teensy 4. The BSP provides access to the Teensy 4's pins and peripherals. It also provides an implementation of the [`log` crate](https://crates.io/crates/log), allowing users to log messages over USB. If you would like to develop Rust applications for the Teensy 4, start here.
- `teensy4-examples`: a collection of examples which run out-of-the-box on the Teensy 4. Take a look at the examples if you're interested in using these crates.
- `teensy4-fcb`: an FCB specific to the Teensy 4. It auto-generates the FCB using the `imxrt1060-fcb-gen` crate.
- `tools`: small Rust binaries intended to run on your development host. `tools` provides a Rust binary that will help import PAC crates. See the "Peripheral access crates" notes (below) for more information.

Although we strive for compatibility with existing crates and frameworks, we've introduced some custom modules in order to operate with the Teensy 4.0. We describe these differences below.

### Runtime

An embedded Rust developer might use the [`cortex-m-rt`](https://crates.io/crates/cortex-m-rt) to bootstrap a Cortex-M system. However, [#164](https://github.com/rust-embedded/cortex-m-rt/issues/164) notes that the `cortex-m-rt` crate cannot yet support devices with custom memory layouts. The iMXRT106x is one of the systems with a custom memory layout; in particular, we have tightly-coupled memory (TCM) regions for instructions (ITCM) and data (DTCM). We also need to place special arrays (the FCB) in memory in order to properly boot. Given these requirements, we need a custom runtime crate that can initialize the system.

The `imxrt1060-rt` crate is a fork of the `cortex-m-rt` crate that is customized to support a minimal iMXRT1060 startup and runtime. Like the `cortex-m-rt` crate, the `imxrt1060-rt` crate

- populates the vector table for correct booting and exception / interrupt dispatch
- initializes static variables
- enables the FPU (since we're a `thumbv7em-none-eabihf` device)

The `imxrt1060-rt` crate goes a step further in its startup functionality:

- provides the required firmware configuration block (FCB) and image vector table (IVT) in order to start the iMXRT106x
- initialize the TCM memory regions
- configures instruction and data caches based on the TCM regions

Just as the `cortex-m-rt` crate will call a user's `main()` function, the `imxrt1060-rt` completes by calling a user's `main()`. The `imxrt1060-rt` crate also exposes the `#[interrupt]`, `#[exception]`, and `#[entry]` macros for decorating interrupt handlers, exception handlers, and the program entrypoint, respectively. Note that, as of this writing, `#[pre_init]` is not supported.

To support compatibility with the `cortex-m-rt` crate, the `imxrt1060-rt` crate uses the same link sections as the `cortex-m-rt` crate. However, the `imxrt1060-rt` crate may locate memory in different regions. Specifically, all instructions are placed into ITCM, and all data is placed into DTCM.

It is our hope that the `imxrt1060-rt` crate can be transparently replaced with the `cortext-m-rt` crate once the necessary features are available. If you think that the `imxrt1060-rt` crate is be diverging from the `cortex-m-rt` crate and might miss that goal, please file an issue!

### Peripheral access crates

An embedded Rust developer might use [`svd2rust`](https://docs.rs/svd2rust/0.16.1/svd2rust/) to auto-generate the peripheral access crate (PAC) for the iMXRT106x. The iMXRT106x SVD is readily [available online](https://developer.arm.com/tools-and-software/embedded/cmsis), and `svd2rust` is able to create the PAC without too many issues. However, [as mpasternacki noted](https://users.rust-lang.org/t/svd2rust-generates-an-enormous-crate/32372), the output PAC from `svd2rust` is extremely large, and it takes an inordinate amount of time to compile. One bottleneck is that the PAC mega-crate cannot be compiled in parallel, since the Rust compiler treats each crate as a translation unit. In order to compile the iMXRT106x peripheral modules in parallel, the peripherals would have to be broken apart into separate, indepdent crates.

The `imxrt1060-pac` crate follows this approach. Rather than using the output of `svd2rust` as the single PAC, we expose each peripheral as its own crate under `imxrt1060-pac`. The approach is semi-automated. We provide a tool (under `tools`) to auto-generate the peripheral crate from the well-formed `svd2rust` output. Once the peripheral crate is added to `imxrt1060-pac`, it's a matter of re-exporting the peripheral in `imxrt1060-pac/src/lib.rs`, un-commenting the corresponding code, and adding the peripheral crate to the workspace. The strategy allows us to add iMXRT106x peripherals as needed, and we benefit from the parallel compilation of the relevant code. The peripheral code is the same as one might find with any PAC generated via `svd2rust`, so there's no additional learning curve to understand the peripheral APIs.

The approach has some limitations: each peripheral crate ends up having its own copy of the types described in `generic.rs`. It also requires that we've generated the original PAC via `svd2rust`. Finally, and most importantly, the approach has not yet shown to scale in practice. Let us know if you have alternative approaches!

The `imxrt1060-core` subcrate defines the interrupt table and interrupt handlers, which is why it's necessary to use it with the `imxrt1060-rt` crate. As mentioned earlier, this requirement should be lifted in the future.

## Contributing

We welcome support! There are known issues that anyone can address in the issues tracker. And, the best way to contribute is to start using the crates to develop applications for the Teensy 4. Submit an issue to help us identify bugs, feature requests, or documentation gaps. If you would like a peripheral crate, let us know, or follow the instructions [here](imxrt1060-pac/README.md) to add the peripheral.

## Q/A

*If they work across all NXP iMXRT106x processors, why are the crates prefixed with `imxrt1060`, and not `imxrt106x`?

When we first used `svd2rust` to create the PAC crate, it listed the PAC as being relevant to the iMXRT1060. We kept the convention. Note that we haven't tested any other iMXRT106x variants besides the variant on the Teensy 4.

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
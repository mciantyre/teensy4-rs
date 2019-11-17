# teensy4-rs

A collection of crates that support the development of Rust applications and libraries for the Teensy 4.

Status: prototype. We can blink the LED, register exceptions, and register interrupts. No one has measured anything. No one has built a fully-fledged application with these crates, yet...

## Dependencies

- A Rust installation. We use the latest, stable Rust compiler. Last tested on Rust 1.39.0. Recommended installation via `rustup`.
- The `thumbv7-none-eabihf` Rust target, which may be installed via `rustup`:

```
$ rustup target add thumbv7em-none-eabihf
```

- The [GNU ARM Embedded Toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm) for compiling the few C sources we need to bootstrap startup (in `teensy4-rt`). Specifically, we need the C compiler and archiver available on our path to build the runtime crate.
- Optionally, a build of [`teensy_loader_cli`](https://github.com/PaulStoffregen/teensy_loader_cli) available on our path. We have a script to rapidly test example programs in the `teensy4-examples`, and it makes use of `teensy_loader_cli`. To load applications onto the Teensy 4, we may also use the [Teensy Loader Application](https://www.pjrc.com/teensy/loader.html), which is also available with the Teensyduino add-ons.

## Getting started

The best way to test our setup is to use the `hardware-test.sh` script to compile one of the example binaries. With a Teensy 4 connected to our system, build and load an example:

```
./hardware-test.sh systick
```

If all goes well, the `systick` example should begin blinking our Teensy 4's LED at ~1Hz.

To begin building our own applications with these crates, include both the `teensy4-rt` and `imxrt1060-pac` crates in a `Cargo.toml`:

```
[dependencies] # Or, specify the git repos to these crates
imxrt1060-pac = { path = "../imxrt1060-pac" }
teensy4-rt = { path = "../teensy4-rt" }
```

See the `teensy4-examples` crate for an example of a project that works on the Teensy 4.

Note that, as of this writing:

- both crates are necessary to successfully build a Teensy 4 Rust application. This requirement should be lifted in the future.
- the crates are not yet published to crates.io, so we must either clone the repo and reference them locally, or reference the two crates via the git repository.

These crates are guaranteed to build when targeting `thumbv7em-none-eabihf`; we do not support
any other targets.

## Project Structure

The project has a model similar to other embedded Rust projects. However, we've introduced some custom modules in order to operate with the Teensy 4.0. We describe these differences below.

### Runtime

An embedded Rust developer might use the [`cortex-m-rt`](https://crates.io/crates/cortex-m-rt) crate for system startup and a minimal runtime. However, [#164](https://github.com/rust-embedded/cortex-m-rt/issues/164) notes that the `cortex-m-rt` crate cannot yet support devices with custom memory layouts. The iMXRT1060 is one of the systems with a custom memory layout; in particular, we have tightly-coupled memory (TCM) regions for instructions (ITCM) and data (DTCM). We also need to place special arrays in memory in order to properly boot. Given these requirements, we need a custom runtime crate that can initialize the system.

The `teensy4-rt` crate is a fork of the `cortex-m-rt` crate that is customized to support a minimal Teensy 4 startup and runtime. Like the `cortex-m-rt` crate, the `teensy4-rt` crate

- populates the vector table for correct booting and exception / interrupt dispatch
- initializes static variables
- enables the FPU (since we're a `thumbv7em-none-eabihf` device)

The `teensy4-rt` crate goes a step further in its startup functionality:

- provides the required firmware configuration block (FCB) and image vector table (IVT) in order to start the iMXRT1060
- initialize the TCM memory regions
- configures instruction and data caches based on the TCM regions
- sets the available GPIOs to fast GPIO mode, and configures the LED

Just as the `cortex-m-rt` crate will call a user's `main()` function, the `teensy4-rt` completes by calling a user's `main()`. The `teensy4-rt` crate also exposes the `#[interrupt]`, `#[exception]`, and `#[entry]` macros for decorating interrupt handlers, exception handlers, and the program entrypoint, respectively. Note that, as of this writing, `#[pre_init]` is not supported. To support the macros, we need a patched version of the `cortex-m-rt-macros` crate. The patched version is automatically used in the `teensy4-rt` crate, so end users need not know this distinction.

To support compatibility with the `cortex-m-rt` crate, the `teensy4-rt` crate uses the same link sections as the `cortex-m-rt` crate. However, the `teensy4-rt` crate may locate memory in different regions. Specifically, all instructions are placed into ITCM, and all data is placed into DTCM.

It is our hope that the `teensy4-rt` crate can be transparently replaced with the `cortext-m-rt` crate once the necessary features are available. If you think that the `teensy4-rt` crate is be diverging from the `cortex-m-rt` crate and might miss that goal, please file an issue!

### Peripheral access crates

An embedded Rust developer might use [`svd2rust`](https://docs.rs/svd2rust/0.16.1/svd2rust/) to auto-generate the peripheral access crate (PAC) for the iMXRT1060. The iMXRT1060 SVD is readily [available online](https://developer.arm.com/tools-and-software/embedded/cmsis), and `svd2rust` is able to create the PAC without too many issues. However, [as mpasternacki noted](https://users.rust-lang.org/t/svd2rust-generates-an-enormous-crate/32372), the output PAC from `svd2rust` is extremely large, and it takes an inordinate amount of time to compile. One bottleneck is that the PAC mega-crate cannot be compiled in parallel, since the Rust compiler treats each crate as a translation unit. In order to compile the iMXRT1060 peripheral modules in parallel, the peripherals would have to be broken apart into separate, indepdent crates.

The `imxrt1060-pac` crate follows this approach. Rather than using the output of `svd2rust` as the PAC, we expose each peripheral as its own crate under `imxrt1060-pac`. The approach is semi-automated. We provide a tool (under `tools`) to auto-generate the peripheral crate from the well-formed `svd2rust` output. Once the peripheral crate is added to `imxrt1060-pac`, it's a matter of re-exporting the peripheral in `imxrt1060-pac/src/lib.rs`, un-commenting the corresponding code, and adding the peripheral crate to the workspace. The strategy allows us to add iMXRT1060 peripherals as needed, and we benefit from the parallel compilation of the relevant code.

The approach has some limitations: each peripheral crate ends up having its own copy of the types described in `generic.rs`. It also requires that we've generated the original PAC via `svd2rust`. Finally, and most importantly, the approach has not yet shown to scale in practice. Let us know if you have alternative approaches!

The `imxrt1060-core` subcrate defines the interrupt table and interrupt handlers, which is why it's necessary to use it with the `teensy4-rt` crate. As mentioned earlier, this requirement should be lifted in the future.

## Contributing

We welcome support! There are known issues that anyone can address in the issues tracker. And, the best way to contribute is to start using the crates to develop applications for the Teensy 4. Submit an issue to help us identify bugs, feature requests, or documentation gaps. If you would like a peripheral crate, let us know.

## Q/A

*Why is it called the `teensy4-rt` crate, and not the `imxrt1060-rt` crate?*

A general iMXRT1060 runtime would need to work across all different hardware configurations. However, the `teensy4-rt` makes some assumptions that we're providing a runtime specifically for the Teensy 4. Therefore, the runtime crate is for Teensy 4 only. Ideally, an `imxrt1060-rt` crate wouldn't exist, since everyone would rally around the existing `cortex-m-rt` crate.

*Is there an iMXRT1060 HAL crate?*

Not yet! That could be built using the various PACs in the `imxrt1060-pac` crate. Defining the HAL could be a future direction of this project.

*Where's the Teensy 4 Rust BSP?*

That doesn't exist yet! The crates provided in this repository may be the foundation for such a BSP crate. Or, a BSP crate would be founded on an iMXRT1060 HAL crate (see above).

*When will this be on crates.io?*

After we evaluate whether or not this is a good or bad approach to developing Rust applications for the Teensy 4, we will either release these crates to crates.io, or recommend an alternative solution.

## Acknowledgements and References

- The [Teensy 4](https://www.pjrc.com/store/teensy40.html) is wonderful, and that's thanks to the hard work of Paul Stoffregen and friends. We can find the Teensy code used in the Arduino plugins [here](https://github.com/PaulStoffregen/cores). The code greatly influenced this library.
- I'm not the only developer tackling the "Rust on Teensy 4" challenge. Check out mpasternacki's work [here](https://gitlab.com/teensy-rs/teensy-4) as an alternative approach towards the same problem.
- The Rust Cortex M team, specifically the [`cortex-m-rt`](https://github.com/rust-embedded/cortex-m-rt) crate.


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
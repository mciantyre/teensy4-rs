# Getting started with Rust on the Teensy 4

<sup>2020-01-03</sup>

**(2020-10-16) This documentation contains outdated information. It is not maintained.**

We keep it for reference. See the project's README for getting started documentation.

---

In this introductory post, we explore how we can write an embedded Rust application for the Teensy 4. The [Teensy 4](https://www.pjrc.com/store/teensy40.html) is an easy-to-use, low-cost development board. Users typically develop applications for the Teensy 4 using Arduino-compatible libraries and tools. However, in this post, we explore how we might write Teensy 4 applications using just Rust. We'll take advantage of the [`teensy4-rs` project][teensy4-rs-repo], a suite of embedded Rust crates that support the Teensy 4, to simplify system bring-up and peripheral access.

By the end of this guide, we will implement the "hello world" of embedded systems: turning on an LED. Specifically, we will prepare a development environment, write a few lines of Rust code that will turn on your Teensy 4's LED, then build and run the application on your Teensy 4. Along the way, we will talk about some of the Rust dependencies that make this possible.

The guide assumes some knowledge of embedded Rust development. If you're already familiar with embedded Rust development, you'll hopefully find things that are familiar. If anything in this guide was confusing, give the [Embedded Rust Book](https://rust-embedded.github.io/book/) a read, then come back to see if things make sense. If things are still unclear, open an issue [here][teensy4-rs-repo] with pain-points and suggestions, and I'll happily update this guide. The guide also assumes that you have a Teensy 4 handy, although it's only necessary if you'd like to see the LED turn on.

## Dependencies

Before we build our hello world demo, we need to install some build dependencies.

We require the `thumbv7em-none-eabihf` Rust target. The Teensy 4 is built around an NXP iMXRT1062 chip, which has an ARM Cortex-M7 processor and a floating-poing unit (FPU). Use `rustup` to easily install the target:

```bash
rustup target add thumbv7em-none-eabihf
```

We also need the [GNU ARM Embedded Toolchain]. If you dig into the Teensy 4 Rust crates, you'll notice a few C sources. The C sources are required to support the boot sequence and the USB logging capabilities. We build these routines from source when we use `cargo build`, and this requires a C compiler. Download the [GNU ARM Embedded Toolchain] for your system, and ensure that the compiler and tools are on your `PATH`. You may also use your system's package manager to install the toolchain.

Finally, if you would like to run our programs on the Teensy 4, you need a Teensy loader to deploy the program to the board. I recommend using the [`teensy_loader_cli`](https://github.com/PaulStoffregen/teensy_loader_cli), a command-line version of the Teensy loader. But, you may also use the [Teensy Loader Application](https://www.pjrc.com/teensy/loader.html) to deploy programs to the Teensy 4. If you already use the [Teensyduino](https://www.pjrc.com/teensy/teensyduino.html) plug-in, you probably have this tool available in the Arduino IDE.

## Getting Started

Clone the `teensy4-rs` repository. As of this writing, not all of the Teensy 4 crates are available on [crates.io]. Before we can publish our Teensy 4 crates, we need some of our dependencies to also be updated on [crates.io]. Keep an eye on [#21](https://github.com/mciantyre/teensy4-rs/issues/21) for updates.

Until then, make a directory to hold your project, and clone the Teensy 4 crates:

```bash
mkdir getting_started && cd getting_started
git clone https://github.com/mciantyre/teensy4-rs.git
```

Next, create a Rust binary in the same directory:

```bash
cargo new --bin teensy4-hello-world
cd teensy4-hello-world
```

The only Teensy 4-specific dependency we need is the `teensy4-bsp` crate. The BSP (board support package, or board support "crate" to keep with Rust conventions) exposes the set of processor peripherals that are physically available on the Teensy 4 development board. The BSP also bundles the runtime (RT) crate and the hardware abstraction layer (HAL), which are needed to boot the system and interface peripherals.

We'll also use two other crates:

- `embedded-hal`, a hardware abstraction layer (HAL) for embedded systems. The `embedded-hal` crate defines a set of traits for hardware peripherals. It has traits for things like digital outputs, I2C and SPI peripherals, and timers. The BSP exposes peripherals that implement the `embedded-hal` traits. Check out the `embedded-hal` crate [here](https://docs.rs/embedded-hal/0.2.3/embedded_hal/).
- `panic-halt`, a crate that implements a simple panic handler. On bare-metal systems, we typically need to implement our own `panic!()` behavior. Rather than implement our own panic handler, we use an off-the-shelf handler. The `panic-halt` handler will halt the processor and fall into an infinite loop in case of panic. We can learn more about embedded Rust panic handlers [here](https://rust-embedded.github.io/book/start/panicking.html).

Given these three dependencies, our `Cargo.toml`'s `[dependencies]` section resembles

```toml
[dependencies]
embedded-hal = "0.2.3"
panic-halt = "0.2.0"

[dependencies.teensy4-bsp]
path = "../teensy4-rs/teensy4-bsp"
```

Next, add a Cargo configuration to set the default build target and override the linker script. Without a default build target, we would need to specify `--target thumbv7em-none-eabihf` every time we use `cargo build`. To take advantage of the special memory regions of the iMXRT1062, we also need to specify the memory layout of the Teensy 4. The runtime crate emits a linker script, called `link.x`, which we can use to link the final program.

Create a configuration file at `.cargo/config` with the following:

```bash 
mkdir .cargo
cat > .cargo/config <<EOT
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
rustflags = ["-C", "link-arg=-Tlink.x"]

EOT
```

With the configuration file in place, we can use `cargo build` like normal.

Finally, we can write Rust code! Update `main.rs` to resemble

```rust
#![no_std]
#![no_main]

extern crate panic_halt;

use teensy4_bsp as bsp;
use bsp::rt::entry;
use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    peripherals.led.set_high().unwrap();
    loop {}
}
```

Let's break it down, line by line:

`#![no_std]` indicates that we're building without the standard library. We only have the dependency-free Rust [`core`](https://doc.rust-lang.org/core/index.html) library available to build our programs. If you see errors noting that the `thumbv7em-none-eabihf` target doesn't have `std` available, double-check to make sure you wrote `#![no_std]`.

`#![no_main]` indicates that we're using a non-standard `main()` function. We define what looks like a `main()` function as a normal Rust function. The `main()` will be called by a special runtime after the system boots.

`extern crate panic_halt;` includes the panic handler that we talked about earlier.

`use teensy4_bsp as bsp;` aliases the `teensy4_bsp` crate as `bsp` for convenience.

`use bsp::rt::entry;` imports the `entry` macro. The BSP includes a special runtime crate, and that runtime crate is re-exported as `rt`. The `entry` macro tells the runtime crate what function represents the user's entry point. In our example, it's the `main()` function.

`use embedded_hal::digital::v2::OutputPin;` includes the `embedded-hal` trait that lets us interact with the Teensy 4's LED. The LED is a GPIO that implements `OutputPin`.

The `#[entry]` macro decorates our `main()` function. Note that the `main()` function has a return type of `-> !`, which means that it never returns. We place an infinite loop, `loop {}`, at the end of `main()` to meet this requirement.

`let peripherals = bsp::Peripherals::take().unwrap();` initializes the BSP's peripherals. Peripherals represent things like timers, PWM outputs, GPIOs, and more. However, our example is focused only on the LED. To guarantee that we only initialize the peripherals once, `take()` method returns an `Option<Peripherals>`. If we were to `take()` the peripherals later, `take()` would return `None`, since we've already performed the initialization. Since it's the first thing that happens in our example, we can feel confident that our `unwrap()` won't cause a panic.

Finally, `peripherals.led.set_high().unwrap()` calls the method defined by the `OutputPin` trait on the LED, turning on the LED. The call could fail, but we just call `unwrap()` to ignore the (unlikely) error. After enabling the LED, we fall into the infinite loop.

Give this a build with `cargo build`, then transform the output using the `objcopy` tool available from our new [GNU ARM Embedded Toolchain]:

```bash
cargo build
arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabihf/debug/teensy4-hello-world hello-world.hex
```

We're ready to deploy our program to the Teensy 4! If you're using the Teensy Loader graphical program, select and deploy the `hello-world.hex` file that was generated from the prior command. If you're using the CLI tool, run

```bash
teensy_loader_cli --mcu=TEENSY40 -w -v hello-world.hex
```

with your Teensy 4 connected to your host. If all goes well, the Teensy 4's LED should turn on!

If you're interested in more examples, check out the capabilities demonstrated in [`teensy4-examples`](https://github.com/mciantyre/teensy4-rs/tree/master/teensy4-examples/src). You'll find an LED example similar to what we just wrote, and a few more advanced examples. In the rest of our getting started guide, we will talk about the dependencies that let us simply implement our hello world program.

## How does it work?

We compiled quite a few dependencies when we said `cargo build`. The crates prefixed with `imxrt102-` or `teensy4-` are necessary to boot the processor, define the processor registers and read / write access of the registers, and define the hardware abstraction layer. Although the crates have various responsibilities, they can be summarized by asking these two questions:

_How is `main()` called?_ Preparing the system and calling `main()` is handled by a combination of the `imxrt1062-rt` and `teensy4-fcb` crates. The former is a fork of the [`cortex-m-rt`](https://github.com/rust-embedded/cortex-m-rt) crate that supports the custom memory layout and boot sequence of the Teensy 4. The latter is a crate that defines the iMXRT1062 firmware configuration block (FCB), an array of semi-magic numbers that needs to be placed in a particular location in flash. The `teensy4-fcb` crate is Teensy 4-specific, but it's defined in terms of a helper crate which lets us generate the FCB in a `build.rs` build script. When we compile and link these crates, the `imxrt1062-rt` crate defines the memory location of the FCB, and the `teensy4-fcb` supplies the FCB implementation. The runtime crate also implements the reset handler. The reset handler (i) prepares the tightly-coupled memory (TCM) regions, (ii) copies text and data into the TCMs, (iii) sets up the interrupt table, and (iv) enables the FPU. Once all setup is complete, the runtime calls the `main()` function, which is identified by the `#[entry]` macro. Steps i and ii are implemented in C; we'll discuss why that is in another blog post. The rest is all written in Rust.

_How does the LED turn on?_ The PAC, HAL, and BSP crates define the processor registers, the behaviors of those perpipherals, and how those peripherals are exposed on the Teensy 4. The `imxrt1062-pac` is a crate composed of many subcrates -- one crate per peripheral -- that define the processor registers and the possible values we might read / write from those registers. The PAC is generated from the chip's SVD file using [`svd2rust`](https://docs.rs/svd2rust/0.17.0/svd2rust/). The `imxrt1062-hal` crate builds on the PAC to expose the peripherals and chip capabilities behind a safer, more convenient API. The crate consists of both an implementation of the `embedded-hal` traits, and also an API that lets us safely configure the system, including configurations like pin muxing and peripheral clocking. Finally, the `teensy4-bsp` re-exports the peripherals from the HAL and performs board-specific configuration. The BSP renames the processor pad `GPIO_B0_03` to something memorable, like `LED`.

## Next steps

We prepared an environment for writing Rust-based applications for the Teensy 4, then implemented the "hello world" of an embedded system for the Teensy 4. In the process, we used a set of Rust crates that simplify system initialization and peripheral access.

There's still much to do! In fact, if you would like to do more than turn on an LED, you might find that there's not much else available. If you're interested in contributing towards the embedded Rust on Teensy 4 story, please reach out. We have many peripherals to implement, a few designs to rethink, and a whole lot of documentation to write. Join us over [here][teensy4-rs-repo].

[crates.io]: https://crates.io/
[GNU ARM Embedded Toolchain]: https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm
[teensy4-rs-repo]: https://github.com/mciantyre/teensy4-rs
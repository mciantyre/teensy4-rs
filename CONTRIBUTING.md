# Contributing

Thanks for contributing towards the `teensy4-rs` project! This guide
lists the best places to report bugs and request features. It also
describes how we develop the `teensy4-rs` project, and how you can help
us build Rust support for the Teensy 4.

## Summary

Much [`teensy4-rs`] project development happens outside of this
repository. `teensy4-rs` contributors also support the [`imxrt-rs`]
project. The `imxrt-rs` project provides

-   the [`imxrt-hal`] crate, which defines the hardware abstraction
    layer (HAL) for the Teensy 4
-   the [`imxrt-boot-gen`] crate, which supports the Teensy 4 runtime

When requesting a feature or reporting a bug, consider if the issue
should be created here, or in one of the other issue trackers described
above. To get an idea of where to submit issues, see [Issues].

Changes in the `teensy4-rs` project typically happen only after changes
to one of those dependent crates. So, your contributions towards the
`teensy4-rs` project may be better suited as contributions to one of
those crates. To understand where you should develop new features or fix
bugs, see [Development].

## Issues

Requesting features and reporting bugs are both great ways of
contributing to the `teensy4-rs` project! To maximize the impact of your
contributions, make sure you use the correct issue tracker. In the
section below, we describe which issue tracker to use for various
feature requests and bug reports.

When in doubt, feel free to report an issue in the `teensy4-rs` repo,
and we will triage the issue.

#### "The Teensy 4 has an \[XYZ\] *peripheral*. I want to add it!/Can you add it?"

We develop new peripherals in the [`imxrt-hal`] crate. Use the
[`imxrt-hal`]'s issue tracker to request a feature.

#### "My Teensy 4's \[I2C\|SPI\|UART\|...\] *peripheral* is broken!"

We fix peripherals in the [`imxrt-hal`] crate. Use the [`imxrt-hal`]'s
issue tracker to report a bug.

#### "How can I do \[XYZ\] with a Teensy 4 *peripheral*?"

These types of questions typically require better documentation in the
[`imxrt-hal`] crate. Ask peripheral-related questions in the
[`imxrt-hal`] issue tracker.

#### "I need to do \[XYZ\] when *booting* the Teensy 4."

Open an issue with either

-   the [`imxrt-boot-gen`] crate, if your question is specific to the
    i.MX RT boot data structure(s)
-   the [`teensy4-rs`] project, if your question is specific to the
    Teensy 4's memory layout and configuration, or the Teensy 4 reset
    handler's behavior

#### "There's a problem when I boot my Teensy 4."

Sorry about that! Open an issue in the [`teensy4-rs`] project.

#### "The [`imxrt-hal`] has \[XYZ\], but I can't seem to make it work with the Teensy 4!"

Open an issue in the [`teensy4-rs`] project. It may be that we haven't
released the feature yet, or we forgot to integrate the HAL's feature in
the Teensy 4.

## Development

To develop the `teensy4-rs` crates, ensure that you have the build
dependencies described in the [README]. We recommend installing (or
building from source) the [`teensy_loader_cli`] command line Teensy
loader, since it's used in some of our automation. The graphical [Teensy
Loader Application] will also work.

Following the "getting started" guide in the [README] to test examples
against your physical Teensy 4. If everything works, you're ready to
contribute! But, before jumping in, make sure that your changes belong
in the `teensy4-rs` project.

If you'd like to add a new peripheral or processor capability, add the
feature to the [`imxrt-hal`] crate. Then, integrate it in the
`teensy4-bsp`. By first adding the peripheral to the `imxrt-hal` crate,
you'll help other users who use the i.MX RT processors in other
projects. See the `imxrt-hal`'s [contributing guidelines] to learn about
`imxrt-hal` development.

If you'd like to change

-   how the Teensy 4 starts up
-   the Teensy 4's memory layout
-   Teensy 4 pin definitions
-   USB logging behaviors

make those changes here, in the [`teensy4-rs`] project.

### Additional Developer Dependencies

We provide precompiled libraries to support the Teensy 4's startup and
USB logging feature. The former represents select startup routines that
are written in assembly. The latter is the majority of the Teensy 4's
USB stack, which is written in C.

To compile the two supporting libraries, you'll need the [GNU ARM
Embedded Toolchain]. Once you have `arm-none-eabi-gcc` on your `PATH`,
you may build both libraries using `make`. Consult `bin/Makefile` for
the relevant targets.

### Workflow

Since most development happens outside of the `teensy4-rs` repo, we
don't maintain a strict workflow in this repo. This section describes
what the `teensy4-rs` contributors are doing today. Note that this
workflow is subject to change.

The main branch describes all upcoming, unreleased development. The main
branch always builds. It may use git dependencies to reference
unreleased dependencies. Prefer to use dependency patches, rather than
directly replacing the dependency, if either option would work.

Tag releases on either the main branch, or the release branch. Create a
release branch when there's a breaking change on the main branch.

Fix bugs on the main branch. Cherry-pick the fix onto the previous
release branch, if applicable.

### References

To contribute towards the `teensy4-rs` project, it helps to be familiar
with

-   embedded system development with Rust. Read [*The Embedded Rust
    Book*] to learn about embedded Rust development.
-   developing C/C++ applications for the [Teensy 4], and the existing
    Teensy software ecosystem.
-   the i.MX RT processor family. See the [resources] in the
    `imxrt-hal`'s contributing guidelines.

  [`teensy4-rs`]: https://github.com/mciantyre/teensy4-rs
  [`imxrt-rs`]: https://github.com/imxrt-rs
  [`imxrt-hal`]: https://github.com/imxrt-rs/imxrt-hal
  [`imxrt-boot-gen`]: https://github.com/imxrt-rs/imxrt-boot-gen
  [Issues]: #issues
  [Development]: #development
  [README]: README.md
  [`teensy_loader_cli`]: https://github.com/PaulStoffregen/teensy_loader_cli
  [Teensy Loader Application]: https://www.pjrc.com/teensy/loader.html
  [contributing guidelines]: https://github.com/imxrt-rs/imxrt-rs/blob/master/CONTRIBUTING.md
  [GNU ARM Embedded Toolchain]: https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm
  [*The Embedded Rust Book*]: https://rust-embedded.github.io/book/intro/index.html
  [Teensy 4]: https://www.pjrc.com/store/teensy40.html
  [resources]: https://github.com/imxrt-rs/imxrt-rs/blob/master/CONTRIBUTING.md#resources

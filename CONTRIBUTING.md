# Contributing

Thanks for contributing towards the `teensy4-rs` project! This guide lists the best places to report bugs and request features. It also describes how we develop the `teensy4-rs` project, and how you can help us build Rust support for the Teensy 4.

## Summary

Much [`teensy4-rs`] project development happens outside of this repository. `teensy4-rs` contributors also support the [`imxrt-rs`] project. The `imxrt-rs` project provides

- the [`imxrt-hal`] crate, which defines the hardware abstraction layer (HAL) for the Teensy 4
- the [`imxrt-boot-gen`] crate, which supports the Teensy 4 runtime

[`teensy4-rs`]: https://github.com/mciantyre/teensy4-rs
[`imxrt-rs`]: https://github.com/imxrt-rs
[`imxrt-hal`]: https://github.com/imxrt-rs/imxrt-rs
[`imxrt-boot-gen`]: https://github.com/imxrt-rs/imxrt-boot-gen

When requesting a feature or reporting a bug, consider if the issue should be created here, or in one of the other issue trackers described above. To get an idea of where to submit issues, see [Issues](#issues).

Changes in the `teensy4-rs` project typically happen only after changes to one of those dependent crates. So, your contributions towards the `teensy4-rs` project may be better suited as contributions to one of those crates. To understand where you should develop new features or fix bugs, see [Development](#development).

## Issues

Requesting features and reporting bugs are both great ways of contributing to the `teensy4-rs` project! To maximize the impact of your contributions, make sure you use the correct issue tracker. In the section below, we describe which issue tracker to use for various feature requests and bug reports.

When in doubt, feel free to report an issue in the `teensy4-rs` repo, and we will triage the issue.

#### "The Teensy 4 has an [XYZ] *peripheral*. I want to add it!/Can you add it?"

We develop new peripherals in the [`imxrt-hal`] crate. Use the [`imxrt-hal`]'s issue tracker to request a feature.

#### "My Teensy 4's [I2C|SPI|UART|...] *peripheral* is broken!"

We fix peripherals in the [`imxrt-hal`] crate. Use the [`imxrt-hal`]'s issue tracker to report a bug.

#### "How can I do [XYZ] with a Teensy 4 *peripheral*?"

These types of questions typically require better documentation in the [`imxrt-hal`] crate. Ask peripheral-related questions in the [`imxrt-hal`] issue tracker.

#### "I need to do [XYZ] when *booting* the Teensy 4."

Open an issue with either

- the [`imxrt-boot-gen`] crate, if your question is specific to the i.MX RT boot data structure(s)
- the [`teensy4-rs`] project, if your question is specific to the Teensy 4's memory layout and configuration, or the Teensy 4 reset handler's behavior

#### "There's a problem when I boot my Teensy 4."

Sorry about that! Open an issue in the [`teensy4-rs`] project.

#### "The [`imxrt-hal`] has [XYZ], but I can't seem to make it work with the Teensy 4!"

Open an issue in the [`teensy4-rs`] project. It may be that we haven't released the feature yet, or we forgot to integrate the HAL's feature in the Teensy 4.

## Development

To develop the `teensy4-rs` crates, ensure that you have the build dependencies described in the [README](README.md). We recommend installing (or building from source) the [`teensy_loader_cli`](https://github.com/PaulStoffregen/teensy_loader_cli) command line Teensy loader, since it's used in some of our automation scripts. The graphical [Teensy Loader Application](https://www.pjrc.com/teensy/loader.html) will also work.

Following the "getting started" guide in the [README](README.md) to test examples against your physical Teensy 4. If everything works, you're ready to contribute! But, before jumping in, make sure that your changes belong in the `teensy4-rs` project.

If you'd like to add a new peripheral or processor capability, add the feature to the [`imxrt-hal`] crate. Then, integrate it in the `teensy4-bsp`. By first adding the peripheral to the `imxrt-hal` crate, you'll help other users who use the i.MX RT processors in other projects. See the `imxrt-hal`'s [contributing guidelines](https://github.com/imxrt-rs/imxrt-rs/blob/master/CONTRIBUTING.md) to learn about `imxrt-hal` development.

If you'd like to change how the Teensy 4 starts up, or the Teensy 4's memory layout, make those changes here, in the [`teensy4-rs`] project.

### Workflow

Since most development happens outside of the `teensy4-rs` repo, we don't maintain a strict workflow in this repo. This section describes what the `teensy4-rs` contributors are doing today. Note that this workflow is subject to change.

- If a change requires an unreleased feature from the [`imxrt-hal`] or [`imxrt-boot-gen`] crates, it happens in the `development` branch. You'll find that `development` is typically ahead of `master`, as it serves as the integration branch for new features and fixes from the dependencies.
  - Longer-lived development may happen in a separate branch. If it depends on unreleased features from a dependency, it will be merged into `development`.
- If a change can be made directly on `master`, it happens in a separate branch. We merge the branch into `master`, then rebase `development` onto `master`.
- Once we release the dependencies, we merge `development` into `master`.

We enforce the following:

- `master` *never* refers to dependencies on the local filesystem. It must always build straight from a `git clone`.
- `development` may reference unreleased dependencies on the local filesystem

If you're having trouble building the `development` branch, make sure that you have any locally-referenced dependencies on your filesystem. We're migrating to [git depdendencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories), but we first need to make it work in [`imxrt-hal`] project... This change may let us revise the workflow, even before we release the `teensy4-rs` crates to crates.io.

Not all of the `teensy4-rs` crates are available on crates.io. Last time we checked, we were waiting for a release of the `cortex-m-rt-macros` crate (see [#12](https://github.com/mciantyre/teensy4-rs/issues/12)), which is required for a release of the `teensy4-rt` and `teensy4-bsp` crates. Until we can release all of the `teensy4-rs` crates, users are expected to depend on `teensy4-rs` crates using git dependencies. We will revise this workflow once we can formally release the `teensy4-rs` crates to crates.io.

### References

To contribute towards the `teensy4-rs` project, it helps to be familiar with

- embedded system development with Rust. Read [*The Embedded Rust Book*] to learn about embedded Rust development.
- developing C/C++ applications for the [Teensy 4], and the existing Teensy software ecosystem.
- the i.MX RT processor family. See the [resources](https://github.com/imxrt-rs/imxrt-rs/blob/master/CONTRIBUTING.md#resources) in the `imxrt-hal`'s contributing guidelines.

[*The Embedded Rust Book*]: https://rust-embedded.github.io/book/intro/index.html
[Teensy 4]: https://www.pjrc.com/store/teensy40.html

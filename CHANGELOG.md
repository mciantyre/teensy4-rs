# Changelog

## Unreleased

This release lets users combine the USB logging system with RTIC. The new
feature required a few breaking changes. The rest of this section notes
the breaking changes, migration tips, and other minor features in this
release.

De-couple the USB logging and SysTick modules. You no longer need the SysTick
timer to use the USB logging system. The BSP features are now independent,
though they're both enabled by default. This change means that you can use
RTIC with USB logging. See the RTIC examples for a demonstration.

**BREAKING** The `"usb-logging"` feature will not implicitly enable the
`"systick"` feature. To fix your build, explicitly add the `"systick"` feature:

```toml
[dependencies.teensy4-bsp]
# features = ["usb-logging"]  # Before
features = ["usb-logging", "systick"]  # After
default-features = false
```

This only affects users who specify `default-features = false`.

**BREAKING** Since USB logging does not need SysTick, the setup functions
do not accept a `&SysTick` reference. The change applies to these two
functions:

- `usb::init`
- `usb::split`

Instead, the functions require the `imxrt_ral`'s `USB1` instance. The example
below shows one way to migrate your code:

```rust
use teensy4_bsp as bsp;

// Before
let systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
bsp::usb::init(&systick, Default::default()).unwrap();

// Now
use bsp::hal::ral::usb::USB1;
bsp::usb::init(USB1::take().unwrap(), Default::default()).unwrap();
```

**BREAKING** The USB `Reader` and `Writer` methods are fallible. Instead of
returning `usize`s, the methods now return `Result<usize, Error>`. See the
documentation to understand the `Error` type, and to learn about the new
method guarantees.

Add a `flush` method to the `usb::Writer` type.

Add `teensy4_bsp::usb::Filter` type alias to simplify USB filter definitions.

## [0.1.0] - 2020-10-16

First release of `teensy4-bsp` to crates.io.

[0.1.0]: https://github.com/mciantyre/teensy4-rs/releases/tag/teensy4-bsp-0.1.0

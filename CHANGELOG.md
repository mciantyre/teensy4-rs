# Changelog

## Unreleased

This release removes the MPU protection regions from the BSP. Users who depend
on MPU protection may specify their own MPU regions using either the `cortex-m`
APIs, or their own implementation.

This release also removes the I-Cache and D-Cache configuration from the BSP.
To re-enable the caches, consider using the `cortex-m` APIs:

```rust
// [dependencies]
// cortex-m = "0.7"

let mut core = cortex_m::Peripherals::take().unwrap();
core.SCB.enable_dcache(&mut core.CPUID);
core.SCB.enable_icache();
```

## [0.2.0] - 2021-01-09

This release lets users combine the USB logging system with RTIC. The new
feature required a few breaking changes. The rest of this section notes
the breaking changes, migration tips, and other minor features in this
release.

De-couple the USB logging and SysTick modules. You no longer need the SysTick
timer to use the USB logging system. The BSP features are now independent,
though they're both enabled by default. This change means that you can use
RTIC with USB logging. See the RTIC examples for a demonstration.

Users may now check USB poll status in their own USB interrupt handler. This
may support more responsive reading from a USB host.

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

**BREAKING** Users must `poll()` the USB driver to coordinate USB I/O. The BSP
does not implement the `USB_OTG1` interrupt handler. If you do not 
repeatedly call `poll()`, or you do not call it fast enough, the USB device may
now work.

Consider calling `poll()` in your own `USB_OTG1` interrupt handler to maintain
compatibility. If using an interrupt handler, make sure to unmask the `USB_OTG1`
interrupt.

The unsafe snippet below should be sufficient to maintain compatibility in your
system. See the documentation and examples for other implementations that control
`unsafe` usage.

```rust
use teensy4_bsp as bsp;
use bsp::interrupt;

#[cortex_m_rt::interrupt]
unsafe fn USB_OTG1() {
    bsp::usb::poll();
}

// Unmask the interrupt after calling the `usb::init`
// or `usb::setup` functions.
unsafe { cortex_m::peripheral::NVIC::unmask(interrupt::USB_OTG1) };
```

Add a `flush` method to the `usb::Writer` type.

Add `teensy4_bsp::usb::Filter` type alias to simplify USB filter definitions.

## [0.1.1] - 2021-01-04

Fix the USB logger so that logging and flushing occur in a critical section.
Before this fix, logging from an interrupt could preempt a logging call that
was modifying transfer descriptors and buffers in the USB stack. This change
should correct logger thread safety.

## [0.1.0] - 2020-10-16

First release of `teensy4-bsp` to crates.io.

[0.2.0]: https://github.com/mciantyre/teensy4-rs/compare/teensy4-bsp-0.1.1...teensy4-bsp-0.2.0
[0.1.1]: https://github.com/mciantyre/teensy4-rs/compare/teensy4-bsp-0.1.0...teensy4-bsp-0.1.1
[0.1.0]: https://github.com/mciantyre/teensy4-rs/releases/tag/teensy4-bsp-0.1.0

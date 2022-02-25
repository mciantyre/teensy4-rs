# Changelog

## Unreleased

The BSP uses `global_asm` to define the reset vector. Requires Rust 1.59.

**BREAKING** Remove the `common`, `t40`, and `t41` modules. Users can access
these modules through the re-export of `teensy4-pin`, simplified as `pins`.

## [0.3.0] - 2021-12-29

**BREAKING** This release removes the `systick` module, and all SYSTICK APIs.
Users who want to use the system timer should prefer the APIs available from
the `cortex_m` crate. See this project's examples for one way to configure
SYSTICK for your system.

**BREAKING** This release removes the MPU protection regions from the BSP. Users who depend
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

**BREAKING** Correct Rust symbol names, following the Rust API guidelines.
These changes affect symbols in the BSP:

- `LED => Led`
- `usb::Error::IO => usb::Error::Io`

**BREAKING** Updated dependencies:

- `cortex-m` to 0.7
- `cortex-m-rt` to 0.7

**BREAKING** Re-export the 0.2 `teensy4-pins` API. See [the
CHANGELOG](teensy4-pins/CHANGELOG.md) for a description of the small breaking
change. `teensy4-bsp` forwards the `teensy4-pins` package as the `pins` module.

*Deprecate* the `common`, `t40`, and `t41` modules. Users should access these
modules through the `teensy4_bsp::pins` module. See the deprecation warnings
for more information.

## [0.2.2] - 2021-12-21

Users can place the heap in DTCM using `dtcm_heap_start()`. This mimics the
behaviors of the 0.2.0 heap placement. See the 0.2.1 changelog note for more
information.

The table below summarizes the change in heap start APIs between 0.2.0 and
0.2.2.

| Heap location | Release 0.2.0  |    Release 0.2.2    |
| ------------- | -------------- | ------------------- |
|     DTCM      | `heap_start()` | `dtcm_heap_start()` |
|    OCRAM2     |     None       |    `heap_start()`   |


## [0.2.1] - 2021-12-19

Move the starting address for the heap into OCRAM2. Previously, the heap
starting address was in DTCM, and it's expected to advance towards the stack.
The relocation into OCRAM2 is consistent with the official Teensy 4 runtime, and
allows you to have a larger heap (and stack, if heap allocation is used). The
heap may occupy all address of OCRAM2 that are not allocated for DMA buffers.
Heap-allocated state no longer benefits from the performance of DTCM.

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

[0.3.0]: https://github.com/mciantyre/teensy4-rs/compare/teensy4-bsp-0.2.2...teensy4-bsp-0.3.0
[0.2.2]: https://github.com/mciantyre/teensy4-rs/compare/teensy4-bsp-0.2.1...teensy4-bsp-0.2.2
[0.2.1]: https://github.com/mciantyre/teensy4-rs/compare/teensy4-bsp-0.2.0...teensy4-bsp-0.2.1
[0.2.0]: https://github.com/mciantyre/teensy4-rs/compare/teensy4-bsp-0.1.1...teensy4-bsp-0.2.0
[0.1.1]: https://github.com/mciantyre/teensy4-rs/compare/teensy4-bsp-0.1.0...teensy4-bsp-0.1.1
[0.1.0]: https://github.com/mciantyre/teensy4-rs/releases/tag/teensy4-bsp-0.1.0

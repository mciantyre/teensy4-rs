//! A Rust board support package (BSP) for the Teensy 4.
//!
//! `teensy4-bsp` supports the following boards:
//!
//! - Teensy 4.0
//! - Teensy 4.1
//! - Teensy MicroMod
//!
//! If you're just getting started with embedded Rust development on the Teensy 4, take
//! a look at [the `board` module](crate::board). This module provides pre-configured drivers
//! and helper functions to define hardware drivers.
//!
//! Peripherals are re-exported from the [`imxrt-hal`](crate::hal)
//! hardware abstraction layer. For more information on drivers, consult the `imxrt-hal` documentation.
//! Note that `imxrt-hal` drivers depend on low-level resources from `imxrt-ral`. For convenience,
//! the BSP also exposes [`imxrt-ral`](crate::ral). Combine `imxrt-hal` and `imxrt-ral` to have full
//! control of your hardware.
//!
//! Finally, the BSP provides a runtime to simplify application development. It exposes board pins through
//! the [`pins`] module.
//!
//! # Features
//!
//! `teensy4-bsp` supports these features.
//!
//! | Flag            |         Description                          |
//! | --------------- | -------------------------------------------- |
//! | `"rt"`          | Adds runtime support using `imxrt-rt`.       |
//!
//! # Runtime
//!
//! When the runtime is enabled, `teensy4-bsp` defines the memory map. In order to use the memory map,
//! you must **link your program with `t4link.x`**.
//!
//! The memory organization includes
//!
//! - 320 KiB of DTCM, comprised of
//!   - a 16 KiB stack.
//!   - the vector table.
//!   - all zero- and runtime-initialized data (`.bss`, `.data`).
//! - 192 KiB of ITCM, containing _all_ instructions (`.text`).
//! - 512 KiB of OCRAM, comprised of
//!   - any uninitialized data (`.uninit`)
//!   - a 16 KiB heap.
//!
//! If the runtime is disabled, then `teensy4-bsp` does no define the memory map, and it does not
//! depend on `imxrt-rt`. Consider disabling the BSP's runtime feature if you want to implement your
//! own runtime, or if you want to use `imxrt-rt` to define your own memory map.
//!
//! ## Environment variable overrides
//!
//! You can override the size of some memory regions by setting environment variables.
//!
//! - To change the *stack* size, set `TEENSY4_STACK_SIZE` when building.
//! - To change the *heap* size, set `TEENSY4_HEAP_SIZE` when building.
//!
//! The examples below show how to set a 4096 byte stack using its environment variable.
//!
//! ```text
//! TEENSY4_STACK_SIZE=4096
//! TEENSY4_STACK_SIZE=4k     # Convenience for multiples of 1024 bytes.
//! TEENSY4_STACK_SIZE=4K     # Equivalent to the above.
//! ```
//!
//! # Notes
//!
//! ## SRTC reset by loader
//!
//! When the SRTC is enabled, setting the board into program mode then using the Teensy Loader
//! application (GUI) to reboot it will set the current time (Unix epoch, but time in local
//! timezone). This will overwrite whatever time you may have previously set and is ambiguous
//! around the backwards daylight savings transition point.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use imxrt_hal as hal;
pub use imxrt_ral as ral;
#[cfg(all(feature = "rt", target_arch = "arm", target_os = "none"))]
#[cfg_attr(docsrs, doc(cfg(feature = "rt")))]
pub use imxrt_rt as rt;

pub use teensy4_pins as pins;

// Need to reference this so that it doesn't get stripped out
use teensy4_fcb as _;

/// Exported for RTIC. Do not use.
#[doc(hidden)]
pub struct Peripherals(ral::Instances);

#[doc(hidden)]
impl Peripherals {
    #[inline]
    pub unsafe fn steal() -> Self {
        Self(board::instances())
    }
}

#[doc(hidden)]
impl From<Peripherals> for ral::Instances {
    #[inline]
    fn from(periphs: Peripherals) -> Self {
        periphs.0
    }
}

/// Exported for RTIC. Do not use.
#[doc(hidden)]
pub use ral::{interrupt, Interrupt, NVIC_PRIO_BITS};

pub mod board;
mod clock_power;

/// SYSTICK external clock frequency.
///
/// This represents the frequency (Hz) of the external clock
/// that can supply SYSTICK.
// See Section 12.3.2.1 of the reference manual. The note
// explains that the 24MHz clock is divided down to 100KHz
// before reaching SYSTICK.
pub const EXT_SYSTICK_HZ: u32 = 100_000;

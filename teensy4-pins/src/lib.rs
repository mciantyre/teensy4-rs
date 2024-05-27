//! Hardware pins for the Teensy 4.0, 4.1 and MicroMod boards
//!
//! `teensy4-pins` is designed to the [`imxrt-iomuxc`] crate. The pins API constrains
//! the processor pads to the ones that are available on the Teensy 4.0, 4.1 and MicroMod. It also
//! exposes type aliases that simplify pin identification in the type system.
//!
//! [`imxrt-iomuxc`]: https://docs.rs/imxrt-iomuxc/0.1/imxrt_iomuxc/
//!
//! Note that this pin API is optional. You are free to configure the pins using the
//! pad identifiers, instead of the physical pin identifiers. Pads are available directly
//! from the `imxrt-iomuxc` crate.
//!
//! # Teensy 4.0
//!
//! To acquire Teensy 4.0 pins, call `t40::from_pads` and provide all
//! of the processor pads:
//!
//! ```
//! use teensy4_pins::t40;
//! # use imxrt_iomuxc::imxrt1060::Pads;
//!
//! let pads = // Handle to all processor pads
//!     # unsafe { Pads::new() };
//! let pins = t40::from_pads(pads);
//! ```
//!
//! # Teensy 4.1
//!
//! The approach is the same as the Teensy 4.0, replacing `t40` with `t41`:
//!
//! ```
//! use teensy4_pins::t41;
//! # use imxrt_iomuxc::imxrt1060::Pads;
//!
//! let pads = // Handle to all processor pads
//!     # unsafe { Pads::new() };
//! let pins = t41::from_pads(pads);
//! ```
//!
//! # Teensy MicroMod
//!
//! The approach is the same as the Teensy 4.0, replacing `t40` with `tmm`:
//!
//! ```
//! use teensy4_pins::tmm;
//! # use imxrt_iomuxc::imxrt1060::Pads;
//!
//! let pads = // Handle to all processor pads
//!     # unsafe { Pads::new() };
//! let pins = tmm::from_pads(pads);
//! ```
//!
//! # Pin configuration
//!
//! Once you have your pad resources, you can configure pull ups, pull downs, and other pad
//! characteristics. See the [`Config`] documentation for all supported features. Use
//! [`configure`] to apply the settings.
//!
//! For example, here's a pull down on pin 7 and an pull up, open drain on pin 9:
//!
//! ```no_run
//! use teensy4_pins::{t40, Config, configure, PullKeeper, OpenDrain};
//! # use imxrt_iomuxc::imxrt1060::Pads;
//!
//! const P7_CONFIG: Config = Config::zero()
//!     .set_pull_keeper(Some(PullKeeper::Pulldown100k));
//!
//! const P9_CONFIG: Config = Config::zero()
//!     .set_pull_keeper(Some(PullKeeper::Pullup22k))
//!     .set_open_drain(OpenDrain::Enabled);
//!
//! let pads = // Handle to all processor pads
//!     # unsafe { Pads::new() };
//! let mut pins = t40::from_pads(pads);
//!
//! configure(&mut pins.p7, P7_CONFIG);
//! configure(&mut pins.p9, P9_CONFIG);
//! ```
//!
//! # Safety
//!
//! The safe APIs expect to work on the only instance of the processor pads. If you don't have that
//! available, or you need more flexibility, use the unsafe [`t40::Pin::new`](t40::Pins::new()),
//! [`t41::Pins::new`](t41::Pins::new()), or [`tmm::Pins::new`](tmm::Pins::new()) constructor methods
//! to create an instance that may be aliasing another handle to the pads or pins.

#![no_std]

pub mod common;
pub mod t40;
pub mod t41;
pub mod tmm;

mod iomuxc {
    pub use imxrt_iomuxc::imxrt1060::*;
    pub use imxrt_iomuxc::ErasedPad;
}

pub use imxrt_iomuxc;

pub use imxrt_iomuxc::{
    configure, Config, DriveStrength, Hysteresis, OpenDrain, PullKeeper, SlewRate, Speed,
};

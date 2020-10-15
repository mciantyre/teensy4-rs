//! FlexSPI Configuration Block (FCB) for the Teensy 4
//!
//! See the [`imxrt-boot-gen`] crate to learn how this was generated.
//!
//! # Usage
//!
//! Add `teensy4-fcb` to your dependencies:
//!
//! ```toml
//! [dependencies]
//! teensy4-fcb = "0.2"
//! ```
//!
//! Properly place the FCB in your program's memory. See the [`FLEXSPI_CONFIGURATION_BLOCK`](static.FLEXSPI_CONFIGURATION_BLOCK.html)
//! declaration below, or the [`imxrt-boot-gen`] documentation, for more information on
//! how you could refer to the FCB. If you're using the `teensy4-rt` runtime crate,
//! you may skip this step. The `teensy4-rt` runtime is designed to this FCB.
//!
//! Make sure that you reference this crate somewhere in your program!
//! Otherwise, it might get removed from the output. Either use
//!
//! ```
//! use teensy4_fcb as _;
//! ```
//! or
//! ```
//! extern crate teensy4_fcb;
//! ```
//!
//! to reference the FCB in either your library or binary.
//!
//! [`imxrt-boot-gen`]: https://docs.rs/imxrt-boot-gen/latest/imxrt_boot_gen/

#![no_std]

include!(concat!(env!("OUT_DIR"), "/fcb.rs"));

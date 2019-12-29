//! Firmware Configuration Block (FCB) for the Teensy 4
//!
//! See the `imxrt1062-fcb-pac` crate for details on how
//! this was generated. See the accompanying `build.rs`
//! for a demonstration of the API.

#![no_std]

include!(concat!(env!("OUT_DIR"), "/fcb.rs"));

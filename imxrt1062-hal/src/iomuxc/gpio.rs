//! Pad definitions
//!
//! The module exposes iMXRT1060 pads and alternatives. For each pad, we implement
//! a series of methods `alt0()` through `alt9()`, depending on the capabilities
//! of the pad.

mod ad_b0;
pub use ad_b0::*;

mod b0;
pub use b0::*;

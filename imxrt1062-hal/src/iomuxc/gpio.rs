//! Pad definitions
//!
//! The module exposes iMXRT1060 pads and alternatives. For each pad, we implement
//! a series of methods `alt0()` through `alt9()`, depending on the capabilities
//! of the pad.

mod ad_b0;
pub use ad_b0::*;

mod ad_b1;
pub use ad_b1::*;

mod b0;
pub use b0::*;

mod b1;
pub use b1::*;

mod emc;
pub use emc::*;

mod sd_b0;
pub use sd_b0::*;

mod sd_b1;
pub use sd_b1::*;

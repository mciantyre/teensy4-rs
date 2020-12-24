//! Teensy 4.1 specific APIs
//!
//! Use [`into_pins`](into_pins()) to constrain the processor pads into the pins available on the Teensy 4.1.
//! If you cannot safely acquire all processor pads, use the unsafe [`Pins::new`](Pins::new())
//! method to generate pins.
//!
//! | Pin  | Pad ID   |  Alt0    |  Alt1        |  Alt2        |  Alt3     |  Alt4        |  Alt5            |  Alt6        |  Alt7   |  Alt8   |  Alt9   |
//! | ---- | -------- | -------- | ------------ | ------------ | --------- | ------------ | ---------------- | ------------ | ------- | ------- | ------- |
//! |  34  |`B1_13`   |          |              |              |           |              |                  |              |         |         |         |
//! |  35  |`B1_12`   |          |              |              |           |              |                  |              |         |         |         |
//! |  36  |`B1_02`   |          |              |              |           |              |                  |              |         |         |         |
//! |  37  |`B1_03`   |          |              |              |           |              |                  |              |         |         |         |
//! |  38  |`AD_B1_12`|          |              |              |           |              |                  |              |         |         |         |
//! |  39  |`AD_B1_13`|          |              |              |           |              |                  |              |         |         |         |
//! |  40  |`AD_B1_04`|          |              |              |           |              |                  |              |         |         |         |
//! |  41  |`AD_B1_05`|          |              |              |           |              |                  |              |         |         |         |

pub use crate::common::*;
use crate::iomuxc::{ad_b1::*, b1::*, ErasedPad};

/// Pin 34 (4.1)
pub type P34 = B1_13;
/// Pin 35 (4.1)
pub type P35 = B1_12;
/// Pin 36 (4.1)
pub type P36 = B1_02;
/// Pin 37 (4.1)
pub type P37 = B1_03;
/// Pin 38 (4.1)
pub type P38 = AD_B1_12;
/// Pin 39 (4.1)
pub type P39 = AD_B1_13;
/// Pin 40 (4.1)
pub type P40 = AD_B1_04;
/// Pin 41 (4.1)
pub type P41 = AD_B1_05;

/// Type-erased Teensy 4.1 pins
///
/// To get pin 13, the LED, index into the 13th element of this array:
/// `erased_pins[13]`.
///
/// Use [`Pins::erase`](Pins::erase()) to erase pin types.
pub type ErasedPins = [ErasedPad; 42];

/// Teensy 4.1 pins
///
/// See [`into_pins`](into_pins()) to safely constrain the processor's pads, and acquire
/// Teensy 4.1 pins. Or, use [`new`](Pins::new()) to unsafely create pins.
pub struct Pins {
    /// Pin 0
    pub p0: P0,
    /// Pin 1
    pub p1: P1,
    /// Pin 2
    pub p2: P2,
    /// Pin 3
    pub p3: P3,
    /// Pin 4
    pub p4: P4,
    /// Pin 5
    pub p5: P5,
    /// Pin 6
    pub p6: P6,
    /// Pin 7
    pub p7: P7,
    /// Pin 8
    pub p8: P8,
    /// Pin 9
    pub p9: P9,
    /// Pin 10
    pub p10: P10,
    /// Pin 11
    pub p11: P11,
    /// Pin 12
    pub p12: P12,
    /// Pin 13
    pub p13: P13,
    /// Pin 14
    pub p14: P14,
    /// Pin 15
    pub p15: P15,
    /// Pin 16
    pub p16: P16,
    /// Pin 17
    pub p17: P17,
    /// Pin 18
    pub p18: P18,
    /// Pin 19
    pub p19: P19,
    /// Pin 20
    pub p20: P20,
    /// Pin 21
    pub p21: P21,
    /// Pin 22
    pub p22: P22,
    /// Pin 23
    pub p23: P23,
    /// Pin 24
    pub p24: P24,
    /// Pin 25
    pub p25: P25,
    /// Pin 26
    pub p26: P26,
    /// Pin 27
    pub p27: P27,
    /// Pin 28
    pub p28: P28,
    /// Pin 29
    pub p29: P29,
    /// Pin 30
    pub p30: P30,
    /// Pin 31
    pub p31: P31,
    /// Pin 32
    pub p32: P32,
    /// Pin 33
    pub p33: P33,
    // END OF COMMON PINS
    /// Pin 34
    pub p34: P34,
    /// Pin 35
    pub p35: P35,
    /// Pin 36
    pub p36: P36,
    /// Pin 37
    pub p37: P37,
    /// Pin 38
    pub p38: P38,
    /// Pin 39
    pub p39: P39,
    /// Pin 40
    pub p40: P40,
    /// Pin 41
    pub p41: P41,
}

/// Constrain the processor pads to the Teensy 4.1 pins
pub const fn into_pins(iomuxc: crate::iomuxc::Pads) -> Pins {
    Pins {
        p0: iomuxc.ad_b0.p03,
        p1: iomuxc.ad_b0.p02,
        p2: iomuxc.emc.p04,
        p3: iomuxc.emc.p05,
        p4: iomuxc.emc.p06,
        p5: iomuxc.emc.p08,
        p6: iomuxc.b0.p10,
        p7: iomuxc.b1.p01,
        p8: iomuxc.b1.p00,
        p9: iomuxc.b0.p11,
        p10: iomuxc.b0.p00,
        p11: iomuxc.b0.p02,
        p12: iomuxc.b0.p01,
        p13: iomuxc.b0.p03,
        p14: iomuxc.ad_b1.p02,
        p15: iomuxc.ad_b1.p03,
        p16: iomuxc.ad_b1.p07,
        p17: iomuxc.ad_b1.p06,
        p18: iomuxc.ad_b1.p01,
        p19: iomuxc.ad_b1.p00,
        p20: iomuxc.ad_b1.p10,
        p21: iomuxc.ad_b1.p11,
        p22: iomuxc.ad_b1.p08,
        p23: iomuxc.ad_b1.p09,
        p24: iomuxc.ad_b0.p12,
        p25: iomuxc.ad_b0.p13,
        p26: iomuxc.ad_b1.p14,
        p27: iomuxc.ad_b1.p15,
        p28: iomuxc.emc.p32,
        p29: iomuxc.emc.p31,
        p30: iomuxc.emc.p37,
        p31: iomuxc.emc.p36,
        p32: iomuxc.b0.p12,
        p33: iomuxc.emc.p07,
        // END OF COMMON PINS
        p34: iomuxc.b1.p13,
        p35: iomuxc.b1.p12,
        p36: iomuxc.b1.p02,
        p37: iomuxc.b1.p03,
        p38: iomuxc.ad_b1.p12,
        p39: iomuxc.ad_b1.p13,
        p40: iomuxc.ad_b1.p04,
        p41: iomuxc.ad_b1.p05,
    }
}

impl Pins {
    /// Create an instance of `Pins` when you do not have a handle
    /// to the processor pads
    ///
    /// # Safety
    ///
    /// Caller must ensure that the pins are not aliased elsewhere in
    /// the program. This could include
    ///
    /// - an existing handle to the `imxrt-iomuxc` pads,
    /// - another instance of `Pins` that was safely acquired
    ///   using [`into_pins`](into_pins()).
    pub const unsafe fn new() -> Self {
        into_pins(crate::iomuxc::Pads::new())
    }

    /// Erase the types of all pins
    pub fn erase(self) -> ErasedPins {
        [
            self.p0.erase(),
            self.p1.erase(),
            self.p2.erase(),
            self.p3.erase(),
            self.p4.erase(),
            self.p5.erase(),
            self.p6.erase(),
            self.p7.erase(),
            self.p8.erase(),
            self.p9.erase(),
            self.p10.erase(),
            self.p11.erase(),
            self.p12.erase(),
            self.p13.erase(),
            self.p14.erase(),
            self.p15.erase(),
            self.p16.erase(),
            self.p17.erase(),
            self.p18.erase(),
            self.p19.erase(),
            self.p20.erase(),
            self.p21.erase(),
            self.p22.erase(),
            self.p23.erase(),
            self.p24.erase(),
            self.p25.erase(),
            self.p26.erase(),
            self.p27.erase(),
            self.p28.erase(),
            self.p29.erase(),
            self.p30.erase(),
            self.p31.erase(),
            self.p32.erase(),
            self.p33.erase(),
            // END OF COMMON PINS
            self.p34.erase(),
            self.p35.erase(),
            self.p36.erase(),
            self.p37.erase(),
            self.p38.erase(),
            self.p39.erase(),
            self.p40.erase(),
            self.p41.erase(),
        ]
    }
}

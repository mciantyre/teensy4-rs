//! Teensy 4.0 specific API
//!
//! ## Pins, Pads and Alternates
//!
//! The sparse table below describes the Teensy 4 pins, the pad ID, and some of the notable alternate functionalities
//! for each pin. We add entries to the table as we add capabilities to the underlying HAL crate. If a pad's alternates
//! are not listed here, consult the iMXRT1060 reference manual.
//!
//! | Pin  | Pad ID   |  Alt0    |  Alt1        |  Alt2        |  Alt3     |  Alt4        |  Alt5            |  Alt6        |  Alt7   |  Alt8   |  Alt9   |
//! | ---- | -------- | -------- | ------------ | ------------ | --------- | ------------ | ---------------- | ------------ | ------- | ------- | ------- |
//! |  0   |`AD_B0_03`|          |              |  `UART6_RX`  |           |`FlexPWM1_1_X`|                  |              |         |         |         |
//! |  1   |`AD_B0_02`|          |              |  `UART6_TX`  |           |`FlexPWM1_0_X`|                  |              |         |         |         |
//! |  2   |`EMC_04`  |          |`FlexPWM4_2_A`|              |           |              |                  |              |         |         |         |
//! |  3   |`EMC_05`  |          |`FlexPWM4_2_B`|              |           |              |                  |              |         |         |         |
//! |  4   |`EMC_06`  |          |`FlexPWM2_0_A`|              |           |              |                  |              |         |         |         |
//! |  5   |`EMC_08`  |          |`FlexPWM2_1_A`|              |           |              |                  |              |         |         |         |
//! |  6   |`B0_10`   |          |              |`FlexPWM2_2_A`|           |              |                  |              |         |         |         |
//! |  7   |`B1_01`   |          |              |  `UART4_RX`  |           |              |                  |`FlexPWM1_3_B`|         |         |         |
//! |  8   |`B1_00`   |          |              |  `UART4_TX`  |           |              |                  |`FlexPWM1_3_A`|         |         |         |
//! |  9   |`B0_11`   |          |              |`FlexPWM2_2_B`|           |              |                  |              |         |         |         |
//! |  10  |`B0_00`   |          |              |              |`SPI4_PCS0`|              |                  |              |         |         |         |
//! |  11  |`B0_02`   |          |              |              |`SPI4_SDO` |              |                  |              |         |         |         |
//! |  12  |`B0_01`   |          |              |              |`SPI4_SDI` |              |                  |              |         |         |         |
//! |  13  |`B0_03`   |          |              |              |`SPI4_SCK` |              |`GPIO2_3` (`LED`) |              |         |         |         |
//! |  14  |`AD_B1_02`|          |              |  `UART2_TX`  |           |              |                  |              |         |         |         |
//! |  15  |`AD_B1_03`|          |              |  `UART2_RX`  |           |              |                  |              |         |         |         |
//! |  16  |`AD_B1_07`|          |`I2C3_SCL`    |  `UART3_RX`  |           |              |                  |              |         |         |         |
//! |  17  |`AD_B1_06`|          |`I2C3_SDA`    |  `UART3_TX`  |           |              |                  |              |         |         |         |
//! |  18  |`AD_B1_01`|          |              |              |`I2C1_SDA` |              |                  |              |         |         |         |
//! |  19  |`AD_B1_00`|          |              |  `UART2_CTS` |`I2C1_SCL` |              |                  |              |         |         |         |
//! |  20  |`AD_B1_10`|          |              |  `UART8_TX`  |           |              |                  |              |         |         |         |
//! |  21  |`AD_B1_11`|          |              |  `UART8_RX`  |           |              |                  |              |         |         |         |
//! |  22  |`AD_B1_08`|          |`FlexPWM4_0_A`|              |           |              |                  |              |         |         |         |
//! |  23  |`AD_B1_09`|          |`FlexPWM4_1_A`|              |           |              |                  |              |         |         |         |
//! |  24  |`AD_B0_12`|`I2C4_SCL`|              |  `UART1_TX`  |           |`FlexPWM1_2_X`|                  |              |         |         |         |
//! |  25  |`AD_B0_13`|`I2C4_SDA`|              |  `UART1_RX`  |           |`FlexPWM1_3_X`|                  |              |         |         |         |
//! |  26  |`AD_B1_14`|          |              |              |           |              |                  |              |         |         |         |
//! |  27  |`AD_B1_15`|          |              |              |           |              |                  |              |         |         |         |
//! |  28  |`EMC_32`  |          |`FlexPWM3_1_B`|  `UART7_RX`  |           |              |                  |              |         |         |         |
//! |  29  |`EMC_31`  |          |`FlexPWM3_1_A`|  `UART7_TX`  |`SPI1_PCS1`|              |                  |              |         |         |         |
//! |  30  |`EMC_37`  |          |              |              |           |              |                  |              |         |         |         |
//! |  31  |`EMC_36`  |          |              |              |           |              |                  |              |         |         |         |
//! |  32  |`B0_12`   |          |              |              |           |              |                  |              |         |         |         |
//! |  33  |`EMC_07`  |          |`FlexPWM2_0_B`|              |           |              |                  |              |         |         |         |
//! |  34  |`SD_B0_03`|          |`FlexPWM1_1_B`|              |           |  `SPI1_SDI`  |                  |              |         |         |         |
//! |  35  |`SD_B0_02`|          |`FlexPWM1_1_A`|              |           |  `SPI1_SDO`  |                  |              |         |         |         |
//! |  36  |`SD_B0_01`|          |`FlexPWM1_0_B`|  `I2C3_SDA`  |           |  `SPI1_PCS0` |                  |              |         |         |         |
//! |  37  |`SD_B0_00`|          |`FlexPWM1_0_A`|  `I2C3_SCL`  |           |  `SPI1_SCK`  |                  |              |         |         |         |
//! |  38  |`SD_B0_05`|          |`FlexPWM1_2_B`|  `UART8_RX`  |           |              |                  |              |         |         |         |
//! |  39  |`SD_B0_04`|          |`FlexPWM1_2_A`|  `UART8_TX`  |           |              |                  |              |         |         |         |
//!
//! References:
//! - [Teensy 4.0 Schematic Diagram](https://www.pjrc.com/teensy/schematic.html)

use crate::hal::iomuxc;

/// Pin 0
pub type P0 = iomuxc::ad_b0::AD_B0_03;
/// Pin 1
pub type P1 = iomuxc::ad_b0::AD_B0_02;
/// Pin 2
pub type P2 = iomuxc::emc::EMC_04;
/// Pin 3
pub type P3 = iomuxc::emc::EMC_05;
/// Pin 4
pub type P4 = iomuxc::emc::EMC_06;
/// Pin 5
pub type P5 = iomuxc::emc::EMC_08;
/// Pin 6
pub type P6 = iomuxc::b0::B0_10;
/// Pin 7
pub type P7 = iomuxc::b1::B1_01;
/// Pin 8
pub type P8 = iomuxc::b1::B1_00;
/// Pin 9
pub type P9 = iomuxc::b0::B0_11;
/// Pin 10
pub type P10 = iomuxc::b0::B0_00;
/// Pin 11
pub type P11 = iomuxc::b0::B0_02;
/// Pin 12
pub type P12 = iomuxc::b0::B0_01;
/// Pin 13
pub type P13 = iomuxc::b0::B0_03;
/// Pin 14
pub type P14 = iomuxc::ad_b1::AD_B1_02;
/// Pin 15
pub type P15 = iomuxc::ad_b1::AD_B1_03;
/// Pin 16
pub type P16 = iomuxc::ad_b1::AD_B1_07;
/// Pin 17
pub type P17 = iomuxc::ad_b1::AD_B1_06;
/// Pin 18
pub type P18 = iomuxc::ad_b1::AD_B1_01;
/// Pin 19
pub type P19 = iomuxc::ad_b1::AD_B1_00;
/// Pin 20
pub type P20 = iomuxc::ad_b1::AD_B1_10;
/// Pin 21
pub type P21 = iomuxc::ad_b1::AD_B1_11;
/// Pin 22
pub type P22 = iomuxc::ad_b1::AD_B1_08;
/// Pin 23
pub type P23 = iomuxc::ad_b1::AD_B1_09;
/// Pin 24
pub type P24 = iomuxc::ad_b0::AD_B0_12;
/// Pin 25
pub type P25 = iomuxc::ad_b0::AD_B0_13;
/// Pin 28
pub type P28 = iomuxc::emc::EMC_32;
/// Pin 29
pub type P29 = iomuxc::emc::EMC_31;
/// Pin 33
pub type P33 = iomuxc::emc::EMC_07;
/// Pin 36
pub type P36 = iomuxc::sd_b0::SD_B0_01;
/// Pin 37
pub type P37 = iomuxc::sd_b0::SD_B0_00;

/// Teensy 4.0 pins
///
/// Pin 13 can be used for several things; one common usage is for the on-board LED.
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
    /// Pin 28
    pub p28: P28,
    /// Pin 29
    pub p29: P29,
    /// Pin 33
    pub p33: P33,
    /// Pin 36
    pub p36: P36,
    /// Pin 37
    pub p37: P37,
}

/// Constrain the processor pads to the Teensy 4.0 pins
pub fn pins(iomuxc: crate::hal::iomuxc::Pads) -> Pins {
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
        p28: iomuxc.emc.p32,
        p29: iomuxc.emc.p31,
        p33: iomuxc.emc.p07,
        p36: iomuxc.sd_b0.p01,
        p37: iomuxc.sd_b0.p00,
    }
}

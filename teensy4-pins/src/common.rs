//! Common pinout
//!
//! The Teensy 4.0 and 4.1 share many pins. This module provides
//! the pins that are common across both boards. For pins that are unique to
//! each board, and to acquire all of a board's pins, see the [`t40`](super::t40)
//! and [`t41`](super::t41) modules.
//!
//! ## Common pin table
//!
//! **This table is incomplete**
//!
//! We believe this table is accurate. But, there may be alternate functions that are not
//! documented, and we're maintaining this table on a best-effort basis. Besides this table,
//! there are two other ways to identify which pads support which peripheral:
//!
//! - study the i.MX RT 1060 Reference Manual. This is the authority on pad configuration.
//! - study the trait implementations for the pad. Select a pin type alias, like [`P0`],
//!   and click-through to its pad documentation (`AD_B0_03`). Notice the listing of `imxrt-iomuxc`
//!   trait implementations. This describes what kinds of functions the pin supports. The constraints
//!   may be enforced by the HAL's APIs.
//!
//! We welcome documentation contributions!
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
//!
//! References:
//! - [Teensy Schematics](https://www.pjrc.com/teensy/schematic.html)

use crate::iomuxc::{ad_b0::*, ad_b1::*, b0::*, b1::*, emc::*};

/// Pin 0 (common)
pub type P0 = AD_B0_03;
/// Pin 1 (common)
pub type P1 = AD_B0_02;
/// Pin 2 (common)
pub type P2 = EMC_04;
/// Pin 3 (common)
pub type P3 = EMC_05;
/// Pin 4 (common)
pub type P4 = EMC_06;
/// Pin 5 (common)
pub type P5 = EMC_08;
/// Pin 6 (common)
pub type P6 = B0_10;
/// Pin 7 (common)
pub type P7 = B1_01;
/// Pin 8 (common)
pub type P8 = B1_00;
/// Pin 9 (common)
pub type P9 = B0_11;
/// Pin 10 (common)
pub type P10 = B0_00;
/// Pin 11 (common)
pub type P11 = B0_02;
/// Pin 12 (common)
pub type P12 = B0_01;
/// Pin 13 (common)
pub type P13 = B0_03;
/// Pin 14 (common)
pub type P14 = AD_B1_02;
/// Pin 15 (common)
pub type P15 = AD_B1_03;
/// Pin 16 (common)
pub type P16 = AD_B1_07;
/// Pin 17 (common)
pub type P17 = AD_B1_06;
/// Pin 18 (common)
pub type P18 = AD_B1_01;
/// Pin 19 (common)
pub type P19 = AD_B1_00;
/// Pin 20 (common)
pub type P20 = AD_B1_10;
/// Pin 21 (common)
pub type P21 = AD_B1_11;
/// Pin 22 (common)
pub type P22 = AD_B1_08;
/// Pin 23 (common)
pub type P23 = AD_B1_09;
/// Pin 24 (common)
pub type P24 = AD_B0_12;
/// Pin 25 (common)
pub type P25 = AD_B0_13;
/// Pin 26 (common)
pub type P26 = AD_B1_14;
/// Pin 27 (common)
pub type P27 = AD_B1_15;
/// Pin 28 (common)
pub type P28 = EMC_32;
/// Pin 29 (common)
pub type P29 = EMC_31;
/// Pin 30 (common)
pub type P30 = EMC_37;
/// Pin 31 (common)
pub type P31 = EMC_36;
/// Pin 32 (common)
pub type P32 = B0_12;
/// Pin 33 (common)
pub type P33 = EMC_07;

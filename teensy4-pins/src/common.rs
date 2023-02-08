//! Common pinout
//!
//! The Teensy boards share many pins. This module provides
//! the pins that are common across both boards. For pins that are unique to
//! each board, and to acquire all of a board's pins, see
//!
//! - [`t40`](super::t40)
//! - [`t41`](super::t41)
//! - [`tmm`](super::tmm)
//!
//! ## Common pin table
//!
//! This table describes all common pins and their possible functions. Besides this table,
//! there are two other ways to identify which pads support which peripheral:
//!
//! - study the i.MX RT 1060 Reference Manual. This is the authority on pad configuration.
//! - study the trait implementations for the pad. Select a pin type alias, like [`P0`],
//!   and click-through to its pad documentation (`AD_B0_03`). Notice the listing of `imxrt-iomuxc`
//!   trait implementations. This describes what kinds of functions the pin supports. The constraints
//!   may be enforced by the HAL's APIs.
//!
//! | Pin | Pad ID        |       Alt0      |       Alt1      |     Alt2        |          Alt3        |         Alt4         |       Alt5         |          Alt6        |       Alt7           |          Alt8         |      Alt9        |
//! | --- | ------------- | --------------- | --------------- | --------------- | -------------------- | -------------------- | ------------------ | -------------------- | -------------------- | --------------------- | ---------------- |
//! |  0  | GPIO_AD_B0_03 | FLEXCAN2_RX     | XBAR1_INOUT17   | LPUART6_RX      | USB_OTG1_OC          | FLEXPWM1_PWMX01      | GPIO1_IO03         | REF_CLK_24M          | LPSPI3_PCS0          | ---                   | ---              |
//! |  1  | GPIO_AD_B0_02 | FLEXCAN2_TX     | XBAR1_INOUT16   | LPUART6_TX      | USB_OTG1_PWR         | FLEXPWM1_PWMX00      | GPIO1_IO02         | LPI2C1_HREQ          | LPSPI3_SDI           | ---                   | ---              |
//! |  2  | GPIO_EMC_04   | SEMC_DATA04     | FLEXPWM4_PWMA02 | SAI2_TX_DATA    | XBAR1_INOUT06        | FLEXIO1_FLEXIO04     | GPIO4_IO04         | ---                  | ---                  | ---                   | ---              |
//! |  3  | GPIO_EMC_05   | SEMC_DATA05     | FLEXPWM4_PWMB02 | SAI2_TX_SYNC    | XBAR1_INOUT07        | FLEXIO1_FLEXIO05     | GPIO4_IO05         | ---                  | ---                  | ---                   | ---              |
//! |  4  | GPIO_EMC_06   | SEMC_DATA06     | FLEXPWM2_PWMA00 | SAI2_TX_BCLK    | XBAR1_INOUT08        | FLEXIO1_FLEXIO06     | GPIO4_IO06         | ---                  | ---                  | ---                   | ---              |
//! |  5  | GPIO_EMC_08   | SEMC_DM00       | FLEXPWM2_PWMA01 | SAI2_RX_DATA    | XBAR1_INOUT17        | FLEXIO1_FLEXIO08     | GPIO4_IO08         | ---                  | ---                  | ---                   | ---              |
//! |  6  | GPIO_B0_10    | LCD_DATA06      | QTIMER4_TIMER1  | FLEXPWM2_PWMA02 | SAI1_TX_DATA03       | FLEXIO2_FLEXIO10     | GPIO2_IO10         | SRC_BOOT_CFG06       | ---                  | ENET2_CRS             | ---              |
//! |  7  | GPIO_B1_01    | LCD_DATA13      | XBAR1_INOUT15   | LPUART4_RX      | SAI1_TX_DATA00       | FLEXIO2_FLEXIO17     | GPIO2_IO17         | FLEXPWM1_PWMB03      | ---                  | ENET2_RDATA00         | FLEXIO3_FLEXIO17 |
//! |  8  | GPIO_B1_00    | LCD_DATA12      | XBAR1_INOUT14   | LPUART4_TX      | SAI1_RX_DATA00       | FLEXIO2_FLEXIO16     | GPIO2_IO16         | FLEXPWM1_PWMA03      | ---                  | ENET2_RX_ER           | FLEXIO3_FLEXIO16 |
//! |  9  | GPIO_B0_11    | LCD_DATA07      | QTIMER4_TIMER2  | FLEXPWM2_PWMB02 | SAI1_TX_DATA02       | FLEXIO2_FLEXIO11     | GPIO2_IO11         | SRC_BOOT_CFG07       | ---                  | ENET2_COL             | ---              |
//! |  10 | GPIO_B0_00    | LCD_CLK         | QTIMER1_TIMER0  | MQS_RIGHT       | LPSPI4_PCS0          | FLEXIO2_FLEXIO00     | GPIO2_IO00         | SEMC_CSX01           | ---                  | ENET2_MDC             | ---              |
//! |  11 | GPIO_B0_02    | LCD_HSYNC       | QTIMER1_TIMER2  | FLEXCAN1_TX     | LPSPI4_SDO           | FLEXIO2_FLEXIO02     | GPIO2_IO02         | SEMC_CSX03           | ---                  | ENET2_1588_EVENT0_OUT | ---              |
//! |  12 | GPIO_B0_01    | LCD_ENABLE      | QTIMER1_TIMER1  | MQS_LEFT        | LPSPI4_SDI           | FLEXIO2_FLEXIO01     | GPIO2_IO01         | SEMC_CSX02           | ---                  | ENET2_MDIO            | ---              |
//! |  13 | GPIO_B0_03    | LCD_VSYNC       | QTIMER2_TIMER0  | FLEXCAN1_RX     | LPSPI4_SCK           | FLEXIO2_FLEXIO03     | GPIO2_IO03 (LED)   | WDOG2_RESET_B_DEB    | ---                  | ENET2_1588_EVENT0_IN  | ---              |
//! |  14 | GPIO_AD_B1_02 | USB_OTG1_ID     | QTIMER3_TIMER2  | LPUART2_TX      | SPDIF_OUT            | ENET_1588_EVENT2_OUT | GPIO1_IO18         | USDHC1_CD_B          | KPP_ROW06            | GPT2_CLK              | FLEXIO3_FLEXIO02 |
//! |  15 | GPIO_AD_B1_03 | USB_OTG1_OC     | QTIMER3_TIMER3  | LPUART2_RX      | SPDIF_IN             | ENET_1588_EVENT2_IN  | GPIO1_IO19         | USDHC2_CD_B          | KPP_COL06            | GPT2_CAPTURE1         | FLEXIO3_FLEXIO03 |
//! |  16 | GPIO_AD_B1_07 | FLEXSPIB_DATA00 | LPI2C3_SCL      | LPUART3_RX      | SPDIF_EXT_CLK        | CSI_HSYNC            | GPIO1_IO23         | USDHC2_DATA3         | KPP_COL04            | GPT2_COMPARE3         | FLEXIO3_FLEXIO07 |
//! |  17 | GPIO_AD_B1_06 | FLEXSPIB_DATA01 | LPI2C3_SDA      | LPUART3_TX      | SPDIF_LOCK           | CSI_VSYNC            | GPIO1_IO22         | USDHC2_DATA2         | KPP_ROW04            | GPT2_COMPARE2         | FLEXIO3_FLEXIO06 |
//! |  18 | GPIO_AD_B1_01 | USB_OTG1_PWR    | QTIMER3_TIMER1  | LPUART2_RTS_B   | LPI2C1_SDA           | CCM_PMIC_READY       | GPIO1_IO17         | USDHC1_VSELECT       | KPP_COL07            | ENET2_1588_EVENT0_IN  | FLEXIO3_FLEXIO01 |
//! |  19 | GPIO_AD_B1_00 | USB_OTG2_ID     | QTIMER3_TIMER0  | LPUART2_CTS     | LPI2C1_SCL           | WDOG1_B              | GPIO1_IO16         | USDHC1_WP            | KPP_ROW07            | ENET2_1588_EVENT0_OUT | FLEXIO3_FLEXIO00 |
//! |  20 | GPIO_AD_B1_10 | FLEXSPIA_DATA03 | WDOG1_B         | LPUART8_TX      | SAI1_RX_SYNC         | CSI_DATA07           | GPIO1_IO26         | USDHC2_WP            | KPP_ROW02            | ENET2_1588_EVENT1_OUT | FLEXIO3_FLEXIO10 |
//! |  21 | GPIO_AD_B1_11 | FLEXSPIA_DATA02 | EWM_OUT_B       | LPUART8_RX      | SAI1_RX_BCLK         | CSI_DATA06           | GPIO1_IO27         | USDHC2_RESET_B       | KPP_COL02            | ENET2_1588_EVENT1_IN  | FLEXIO3_FLEXIO11 |
//! |  22 | GPIO_AD_B1_08 | FLEXSPIA_SS1_B  | FLEXPWM4_PWMA00 | FLEXCAN1_TX     | CCM_PMIC_READY       | CSI_DATA09           | GPIO1_IO24         | USDHC2_CMD           | KPP_ROW03            | ---                   | FLEXIO3_FLEXIO08 |
//! |  23 | GPIO_AD_B1_09 | FLEXSPIA_DQS    | FLEXPWM4_PWMA01 | FLEXCAN1_RX     | SAI1_MCLK            | CSI_DATA08           | GPIO1_IO25         | USDHC2_CLK           | KPP_COL03            | ---                   | FLEXIO3_FLEXIO09 |
//! |  24 | GPIO_AD_B0_12 | LPI2C4_SCL      | CCM_PMIC_READY  | LPUART1_TX      | WDOG2_WDOG_B         | FLEXPWM1_PWMX02      | GPIO1_IO12         | ENET_1588_EVENT1_OUT | NMI_GLUE_NMI         | ---                   | ---              |
//! |  25 | GPIO_AD_B0_13 | LPI2C4_SDA      | GPT1_CLK        | LPUART1_RX      | EWM_OUT_B            | FLEXPWM1_PWMX03      | GPIO1_IO13         | ENET_1588_EVENT1_IN  | REF_CLK_24M          | ---                   | ---              |
//! |  26 | GPIO_AD_B1_14 | USB_OTG2_OC     | XBAR1_IN24      | LPUART1_CTS_B   | ENET_1588_EVENT0_OUT | CSI_VSYNC            | GPIO1_IO14         | FLEXCAN2_TX          | ---                  | FLEXCAN3_TX           | ---              |
//! |  27 | GPIO_AD_B1_15 | USB_OTG2_PWR    | XBAR1_IN25      | LPUART1_RTS_B   | ENET_1588_EVENT0_IN  | CSI_HSYNC            | GPIO1_IO15         | FLEXCAN2_RX          | WDOG1_WDOG_RST_B_DEB | FLEXCAN3_RX           | ---              |
//! |  28 | GPIO_EMC_32   | SEMC_DATA10     | FLEXPWM3_PWMB01 | LPUART7_RX      | CCM_PMIC_RDY         | CSI_DATA21           | GPIO3_IO18         | ---                  | ---                  | ENET2_TX_EN           | ---              |
//! |  29 | GPIO_EMC_31   | SEMC_DATA09     | FLEXPWM3_PWMA01 | LPUART7_TX      | LPSPI1_PCS1          | CSI_DATA22           | GPIO4_IO31         | ---                  | ---                  | ENET2_TDATA01         | ---              |
//! |  30 | GPIO_EMC_37   | SEMC_DATA15     | XBAR1_IN23      | GPT1_COMPARE3   | SAI3_MCLK            | CSI_DATA16           | GPIO3_IO23         | USDHC2_WP            | ---                  | ENET2_RX_EN           | FLEXCAN3_RX      |
//! |  31 | GPIO_EMC_36   | SEMC_DATA14     | XBAR1_IN22      | GPT1_COMPARE2   | SAI3_TX_DATA         | CSI_DATA17           | GPIO3_IO22         | USDHC1_WP            | ---                  | ENET2_RDATA01         | FLEXCAN3_TX      |
//! |  32 | GPIO_B0_12    | LCD_DATA08      | XBAR1_INOUT10   | ARM_TRACE_CLK   | SAI1_TX_DATA01       | FLEXIO2_FLEXIO12     | GPIO2_IO12         | SRC_BOOT_CFG08       | ---                  | ENET2_TDATA00         | ---              |
//! |  33 | GPIO_EMC_07   | SEMC_DATA07     | FLEXPWM2_PWMB00 | SAI2_MCLK       | XBAR1_INOUT09        | FLEXIO1_FLEXIO07     | GPIO4_IO07         | ---                  | ---                  | ---                   | ---              |
//!
//! References:
//! - [Teensy Schematics](https://www.pjrc.com/teensy/schematic.html)
//! - i.MX RT 1060 Processor Reference Manual, Rev. 2, 12/2019

use crate::iomuxc::{gpio_ad_b0::*, gpio_ad_b1::*, gpio_b0::*, gpio_b1::*, gpio_emc::*};

/// Pin 0 (common)
pub type P0 = GPIO_AD_B0_03;
/// Pin 1 (common)
pub type P1 = GPIO_AD_B0_02;
/// Pin 2 (common)
pub type P2 = GPIO_EMC_04;
/// Pin 3 (common)
pub type P3 = GPIO_EMC_05;
/// Pin 4 (common)
pub type P4 = GPIO_EMC_06;
/// Pin 5 (common)
pub type P5 = GPIO_EMC_08;
/// Pin 6 (common)
pub type P6 = GPIO_B0_10;
/// Pin 7 (common)
pub type P7 = GPIO_B1_01;
/// Pin 8 (common)
pub type P8 = GPIO_B1_00;
/// Pin 9 (common)
pub type P9 = GPIO_B0_11;
/// Pin 10 (common)
pub type P10 = GPIO_B0_00;
/// Pin 11 (common)
pub type P11 = GPIO_B0_02;
/// Pin 12 (common)
pub type P12 = GPIO_B0_01;
/// Pin 13 (common)
pub type P13 = GPIO_B0_03;
/// Pin 14 (common)
pub type P14 = GPIO_AD_B1_02;
/// Pin 15 (common)
pub type P15 = GPIO_AD_B1_03;
/// Pin 16 (common)
pub type P16 = GPIO_AD_B1_07;
/// Pin 17 (common)
pub type P17 = GPIO_AD_B1_06;
/// Pin 18 (common)
pub type P18 = GPIO_AD_B1_01;
/// Pin 19 (common)
pub type P19 = GPIO_AD_B1_00;
/// Pin 20 (common)
pub type P20 = GPIO_AD_B1_10;
/// Pin 21 (common)
pub type P21 = GPIO_AD_B1_11;
/// Pin 22 (common)
pub type P22 = GPIO_AD_B1_08;
/// Pin 23 (common)
pub type P23 = GPIO_AD_B1_09;
/// Pin 24 (common)
pub type P24 = GPIO_AD_B0_12;
/// Pin 25 (common)
pub type P25 = GPIO_AD_B0_13;
/// Pin 26 (common)
pub type P26 = GPIO_AD_B1_14;
/// Pin 27 (common)
pub type P27 = GPIO_AD_B1_15;
/// Pin 28 (common)
pub type P28 = GPIO_EMC_32;
/// Pin 29 (common)
pub type P29 = GPIO_EMC_31;
/// Pin 30 (common)
pub type P30 = GPIO_EMC_37;
/// Pin 31 (common)
pub type P31 = GPIO_EMC_36;
/// Pin 32 (common)
pub type P32 = GPIO_B0_12;
/// Pin 33 (common)
pub type P33 = GPIO_EMC_07;

//! Teensy 4.1 specific APIs
//!
//! Use [`from_pads`](from_pads()) to constrain the processor pads into the pins available on the Teensy 4.1.
//! If you cannot safely acquire all processor pads, use the unsafe [`Pins::new`](Pins::new())
//! method to generate pins.
//!
//! | Pin  | Pad ID        |      Alt0       |   Alt1          |    Alt2       |        Alt3          |     Alt4         |  Alt5      |     Alt6        |    Alt7   |     Alt8              |      Alt9        |
//! | ---- | ------------- | --------------- | --------------- | ------------- | -------------------- | ---------------- | ---------- | --------------- | --------- | --------------------- | ---------------- |
//! |  34  | GPIO_B1_13    | WDOG1_B         | LPUART5_RX      | CSI_VSYNC     | ENET_1588_EVENT0_OUT | FLEXIO2_FLEXIO29 | GPIO2_IO29 | USDHC1_WP       | ---       | SEMC_DQS4             | FLEXIO3_FLEXIO29 |
//! |  35  | GPIO_B1_12    | ---             | LPUART5_TX      | CSI_PIXCLK    | ENET_1588_EVENT0_IN  | FLEXIO2_FLEXIO28 | GPIO2_IO28 | USDHC1_CD_B     | ---       | ---                   | FLEXIO3_FLEXIO28 |
//! |  36  | GPIO_B1_02    | LCD_DATA14      | XBAR1_INOUT16   | LPSPI4_PCS2   | SAI1_TX_BCLK         | FLEXIO2_FLEXIO18 | GPIO2_IO18 | FLEXPWM2_PWMA03 | ---       | ENET2_RDATA01         | FLEXIO3_FLEXIO18 |
//! |  37  | GPIO_B1_03    | LCD_DATA15      | XBAR1_INOUT17   | LPSPI4_PCS1   | SAI1_TX_SYNC         | FLEXIO2_FLEXIO19 | GPIO2_IO19 | FLEXPWM2_PWMB03 | ---       | ENET2_RX_EN           | FLEXIO3_FLEXIO19 |
//! |  38  | GPIO_AD_B1_12 | FLEXSPIA_DATA01 | ACMP_OUT00      | LPSPI3_PCS0   | SAI1_RX_DATA00       | CSI_DATA05       | GPIO1_IO28 | USDHC2_DATA4    | KPP_ROW01 | ENET2_1588_EVENT2_OUT | FLEXIO3_FLEXIO12 |
//! |  39  | GPIO_AD_B1_13 | FLEXSPIA_DATA00 | ACMP_OUT01      | LPSPI3_SDI    | SAI1_TX_DATA00       | CSI_DATA04       | GPIO1_IO29 | USDHC2_DATA5    | KPP_COL01 | ENET2_1588_EVENT2_IN  | FLEXIO3_FLEXIO13 |
//! |  40  | GPIO_AD_B1_04 | FLEXSPIB_DATA03 | ENET_MDC        | LPUART3_CTS_B | SPDIF_SR_CLK         | CSI_PIXCLK       | GPIO1_IO20 | USDHC2_DATA0    | KPP_ROW05 | GPT2_CAPTURE2         | FLEXIO3_FLEXIO04 |
//! |  41  | GPIO_AD_B1_05 | FLEXSPIB_DATA02 | ENET_MDIO       | LPUART3_RTS_B | SPDIF_OUT            | CSI_MCLK         | GPIO1_IO21 | USDHC2_DATA1    | KPP_COL05 | GPT2_COMPARE1         | FLEXIO3_FLEXIO05 |
//! |  42  | GPIO_SD_B0_03 | USDHC1_DATA1    | FLEXPWM1_PWMB01 | LPUART8_RTS_B | XBAR1_INOUT07        | LPSPI1_SDI       | GPIO3_IO15 | ---             | ---       | ENET2_RDATA00         | SEMC_CLK6        |
//! |  43  | GPIO_SD_B0_02 | USDHC1_DATA0    | FLEXPWM1_PWMA01 | LPUART8_CTS_B | XBAR1_INOUT06        | LPSPI1_SDO       | GPIO3_IO14 | ---             | ---       | ENET2_RX_ER           | SEMC_CLK5        |
//! |  44  | GPIO_SD_B0_01 | USDHC1_CLK      | FLEXPWM1_PWMB00 | LPI2C3_SDA    | XBAR1_INOUT05        | LPSPI1_PCS0      | GPIO3_IO13 | FLEXSPIB_SS1_B  | ---       | ENET2_TX_CLK          | ENET2_REF_CLK2   |
//! |  45  | GPIO_SD_B0_00 | USDHC1_CMD      | FLEXPWM1_PWMA00 | LPI2C3_SCL    | XBAR1_INOUT04        | LPSPI1_SCK       | GPIO3_IO12 | FLEXSPIA_SS1_B  | ---       | ENET2_TX_EN           | SEMC_DQS4        |
//! |  46  | GPIO_SD_B0_05 | USDHC1_DATA3    | FLEXPWM1_PWMB02 | LPUART8_RX    | XBAR1_INOUT09        | FLEXSPIB_DQS     | GPIO3_IO17 | CCM_CLKO2       | ---       | ENET2_RX_EN           | ---              |
//! |  47  | GPIO_SD_B0_04 | USDHC1_DATA2    | FLEXPWM1_PWMA02 | LPUART8_TX    | XBAR1_INOUT08        | FLEXSPIB_SS0_B   | GPIO3_IO16 | CCM_CLKO1       | ---       | ENET2_RDATA01         | ---              |
//! |  48  | GPIO_EMC_24   | SEMC_CAS        | FLEXPWM1_PWMB00 | LPUART5_RX    | ENET_TX_EN           | GPT1_CAPTURE1    | GPIO4_IO24 | ---             | ---       | FLEXSPI2_A_SS0_B      | ---              |
//! |  49  | GPIO_EMC_27   | SEMC_CKE        | FLEXPWM1_PWMA02 | LPUART5_RTS_B | LPSPI1_SCK           | FLEXIO1_FLEXIO13 | GPIO4_IO27 | ---             | ---       | FLEXSPI2_A_DATA01     | ---              |
//! |  50  | GPIO_EMC_28   | SEMC_WE         | FLEXPWM1_PWMB02 | LPUART5_CTS_B | LPSPI1_SDO           | FLEXIO1_FLEXIO14 | GPIO4_IO28 | ---             | ---       | FLEXSPI2_A_DATA02     | ---              |
//! |  51  | GPIO_EMC_22   | SEMC_BA1        | FLEXPWM3_PWMB03 | LPI2C3_SCL    | ENET_TDATA00         | QTIMER2_TIMER3   | GPIO4_IO22 | ---             | ---       | FLEXSPI2_A_SS1_B      | ---              |
//! |  52  | GPIO_EMC_26   | SEMC_CLK        | FLEXPWM1_PWMB01 | LPUART6_RX    | ENET_RX_ER           | FLEXIO1_FLEXIO12 | GPIO4_IO26 | ---             | ---       | FLEXSPI2_A_DATA00     | ---              |
//! |  53  | GPIO_EMC_25   | SEMC_RAS        | FLEXPWM1_PWMA01 | LPUART6_TX    | ENET_TX_CLK          | ENET_REF_CLK     | GPIO4_IO25 | ---             | ---       | FLEXSPI2_A_SCLK       | ---              |
//! |  54  | GPIO_EMC_29   | SEMC_CS0        | FLEXPWM3_PWMA00 | LPUART6_RTS_B | LPSPI1_SDI           | FLEXIO1_FLEXIO15 | GPIO4_IO29 | ---             | ---       | FLEXSPI2_A_DATA03     | ---              |

pub use crate::common::*;
use crate::iomuxc::{gpio_ad_b1::*, gpio_b1::*, gpio_emc::*, gpio_sd_b0::*, ErasedPad};

/// Pin 34 (4.1)
pub type P34 = GPIO_B1_13;
/// Pin 35 (4.1)
pub type P35 = GPIO_B1_12;
/// Pin 36 (4.1)
pub type P36 = GPIO_B1_02;
/// Pin 37 (4.1)
pub type P37 = GPIO_B1_03;
/// Pin 38 (4.1)
pub type P38 = GPIO_AD_B1_12;
/// Pin 39 (4.1)
pub type P39 = GPIO_AD_B1_13;
/// Pin 40 (4.1)
pub type P40 = GPIO_AD_B1_04;
/// Pin 41 (4.1)
pub type P41 = GPIO_AD_B1_05;
/// Pin 42 (4.1)
pub type P42 = GPIO_SD_B0_03;
/// Pin 43 (4.1)
pub type P43 = GPIO_SD_B0_02;
/// Pin 44 (4.1)
pub type P44 = GPIO_SD_B0_01;
/// Pin 45 (4.1)
pub type P45 = GPIO_SD_B0_00;
/// Pin 46 (4.1)
pub type P46 = GPIO_SD_B0_05;
/// Pin 47 (4.1)
pub type P47 = GPIO_SD_B0_04;
/// Pin 48 (4.1)
pub type P48 = GPIO_EMC_24;
/// Pin 49 (4.1)
pub type P49 = GPIO_EMC_27;
/// Pin 50 (4.1)
pub type P50 = GPIO_EMC_28;
/// Pin 51 (4.1)
pub type P51 = GPIO_EMC_22;
/// Pin 52 (4.1)
pub type P52 = GPIO_EMC_26;
/// Pin 53 (4.1)
pub type P53 = GPIO_EMC_25;
/// Pin 54 (4.1)
pub type P54 = GPIO_EMC_29;

/// Type-erased Teensy 4.1 pins
///
/// To get pin 13, the LED, index into the 13th element of this array:
/// `erased_pins[13]`.
///
/// Use [`Pins::erase`](Pins::erase()) to erase pin types.
pub type ErasedPins = [ErasedPad; 55];

/// Teensy 4.1 pins
///
/// See [`from_pads`](from_pads()) to safely constrain the processor's pads, and acquire
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
    /// Pin 42
    pub p42: P42,
    /// Pin 43
    pub p43: P43,
    /// Pin 44
    pub p44: P44,
    /// Pin 45
    pub p45: P45,
    /// Pin 46
    pub p46: P46,
    /// Pin 47
    pub p47: P47,
    /// Pin 48
    pub p48: P48,
    /// Pin 49
    pub p49: P49,
    /// Pin 50
    pub p50: P50,
    /// Pin 51
    pub p51: P51,
    /// Pin 52
    pub p52: P52,
    /// Pin 53
    pub p53: P53,
    /// Pin 54
    pub p54: P54,
}

/// Constrain the processor pads to the Teensy 4.1 pins
#[inline]
pub const fn from_pads(iomuxc: crate::iomuxc::Pads) -> Pins {
    Pins {
        p0: iomuxc.gpio_ad_b0.p03,
        p1: iomuxc.gpio_ad_b0.p02,
        p2: iomuxc.gpio_emc.p04,
        p3: iomuxc.gpio_emc.p05,
        p4: iomuxc.gpio_emc.p06,
        p5: iomuxc.gpio_emc.p08,
        p6: iomuxc.gpio_b0.p10,
        p7: iomuxc.gpio_b1.p01,
        p8: iomuxc.gpio_b1.p00,
        p9: iomuxc.gpio_b0.p11,
        p10: iomuxc.gpio_b0.p00,
        p11: iomuxc.gpio_b0.p02,
        p12: iomuxc.gpio_b0.p01,
        p13: iomuxc.gpio_b0.p03,
        p14: iomuxc.gpio_ad_b1.p02,
        p15: iomuxc.gpio_ad_b1.p03,
        p16: iomuxc.gpio_ad_b1.p07,
        p17: iomuxc.gpio_ad_b1.p06,
        p18: iomuxc.gpio_ad_b1.p01,
        p19: iomuxc.gpio_ad_b1.p00,
        p20: iomuxc.gpio_ad_b1.p10,
        p21: iomuxc.gpio_ad_b1.p11,
        p22: iomuxc.gpio_ad_b1.p08,
        p23: iomuxc.gpio_ad_b1.p09,
        p24: iomuxc.gpio_ad_b0.p12,
        p25: iomuxc.gpio_ad_b0.p13,
        p26: iomuxc.gpio_ad_b1.p14,
        p27: iomuxc.gpio_ad_b1.p15,
        p28: iomuxc.gpio_emc.p32,
        p29: iomuxc.gpio_emc.p31,
        p30: iomuxc.gpio_emc.p37,
        p31: iomuxc.gpio_emc.p36,
        p32: iomuxc.gpio_b0.p12,
        p33: iomuxc.gpio_emc.p07,
        // END OF COMMON PINS
        p34: iomuxc.gpio_b1.p13,
        p35: iomuxc.gpio_b1.p12,
        p36: iomuxc.gpio_b1.p02,
        p37: iomuxc.gpio_b1.p03,
        p38: iomuxc.gpio_ad_b1.p12,
        p39: iomuxc.gpio_ad_b1.p13,
        p40: iomuxc.gpio_ad_b1.p04,
        p41: iomuxc.gpio_ad_b1.p05,
        p42: iomuxc.gpio_sd_b0.p03,
        p43: iomuxc.gpio_sd_b0.p02,
        p44: iomuxc.gpio_sd_b0.p01,
        p45: iomuxc.gpio_sd_b0.p00,
        p46: iomuxc.gpio_sd_b0.p05,
        p47: iomuxc.gpio_sd_b0.p04,
        p48: iomuxc.gpio_emc.p24,
        p49: iomuxc.gpio_emc.p27,
        p50: iomuxc.gpio_emc.p28,
        p51: iomuxc.gpio_emc.p22,
        p52: iomuxc.gpio_emc.p26,
        p53: iomuxc.gpio_emc.p25,
        p54: iomuxc.gpio_emc.p29,
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
    ///   using [`from_pads`](from_pads()).
    #[inline]
    pub const unsafe fn new() -> Self {
        from_pads(crate::iomuxc::Pads::new())
    }

    /// Erase the types of all pins
    #[inline]
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
            self.p42.erase(),
            self.p43.erase(),
            self.p44.erase(),
            self.p45.erase(),
            self.p46.erase(),
            self.p47.erase(),
            self.p48.erase(),
            self.p49.erase(),
            self.p50.erase(),
            self.p51.erase(),
            self.p52.erase(),
            self.p53.erase(),
            self.p54.erase(),
        ]
    }
}

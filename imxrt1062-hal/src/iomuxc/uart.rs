//! UART pin multiplexing

pub mod module {
    use crate::pac;
    pub trait Module {
        const IDX: usize;
        type Reg: core::ops::Deref<Target = pac::lpuart1::RegisterBlock>;
    }
    pub struct _1;
    impl Module for _1 {
        const IDX: usize = 1;
        type Reg = pac::LPUART1;
    }
    pub struct _2;
    impl Module for _2 {
        const IDX: usize = 2;
        type Reg = pac::LPUART2;
    }
    pub struct _3;
    impl Module for _3 {
        const IDX: usize = 3;
        type Reg = pac::LPUART3;
    }
    pub struct _4;
    impl Module for _4 {
        const IDX: usize = 4;
        type Reg = pac::LPUART4;
    }
    pub struct _5;
    impl Module for _5 {
        const IDX: usize = 5;
        type Reg = pac::LPUART5;
    }
    pub struct _6;
    impl Module for _6 {
        const IDX: usize = 6;
        type Reg = pac::LPUART6;
    }
    pub struct _7;
    impl Module for _7 {
        const IDX: usize = 7;
        type Reg = pac::LPUART7;
    }
    pub struct _8;
    impl Module for _8 {
        const IDX: usize = 8;
        type Reg = pac::LPUART8;
    }
}

pub trait Direction {}
pub struct TX;
impl Direction for TX {}
pub struct RX;
impl Direction for RX {}

pub trait Pin {
    type Direction: Direction;
    type Module: module::Module;

    /// Perform IOMUXC configurations for this pin
    fn configure(&mut self);
}

macro_rules! _rx_config {
    // Pad RX configuration and daisy selection
    ($daisy_reg:ident, $daisy_value:ident) => {
        #[inline(always)]
        fn configure(&mut self) {
            self.iomuxc().$daisy_reg.write(|w| w.daisy().$daisy_value());
            self.pad().write(|w| {
                w.dse()
                .dse_7_r0_7()
                .pke()
                .pke_1_pull_keeper_enabled()
                .pue()
                .pue_1_pull()
                .pus()
                .pus_3_22k_ohm_pull_up()
                .hys()
                .hys_1_hysteresis_enabled()
            })
        }
    };
    // Only RX pad configuration
    () => {
        #[inline(always)]
        fn configure(&mut self) {
            self.pad().write(|w| {
                w.dse()
                .dse_7_r0_7()
                .pke()
                .pke_1_pull_keeper_enabled()
                .pue()
                .pue_1_pull()
                .pus()
                .pus_3_22k_ohm_pull_up()
                .hys()
                .hys_1_hysteresis_enabled()
            })
        }
    };
}

macro_rules! _tx_config {
    // TX Pad configuration and daisy selection
    ($daisy_reg:ident, $daisy_value:ident) => {
        #[inline(always)]
        fn configure(&mut self) {
            self.iomuxc().$daisy_reg.write(|w| w.daisy().$daisy_value());
            self.pad().write(|w| {
                w.sre()
                    .sre_1_fast_slew_rate()
                    .dse()
                    .dse_3_r0_3()
                    .speed()
                    .speed_3_max_200mhz()
            });
        }
    };
    // Only TX pad configuration
    () => {
        #[inline(always)]
        fn configure(&mut self) {
            self.pad().write(|w| {
                w.sre()
                    .sre_1_fast_slew_rate()
                    .dse()
                    .dse_3_r0_3()
                    .speed()
                    .speed_3_max_200mhz()
            });
        }
    };
}

use crate::iomuxc::{
    gpio::{
        GPIO_AD_B0_02, GPIO_AD_B0_03, GPIO_AD_B0_12, GPIO_AD_B0_13, GPIO_AD_B1_02, GPIO_AD_B1_03,
        GPIO_AD_B1_06, GPIO_AD_B1_07, GPIO_AD_B1_10, GPIO_AD_B1_11, GPIO_B1_00, GPIO_B1_01,
        GPIO_EMC_31, GPIO_EMC_32,
    },
    Alt2,
};

macro_rules! _impl_rx {
    // Implement a RX pad that needs daisy selection
    ($pad:ty, $module:ty, $daisy_reg:ident, $daisy_val:ident) => {
        impl Pin for $pad {
            type Direction = RX;
            type Module = $module;

            _rx_config!($daisy_reg, $daisy_val);
        }
    };
    // Implement a RX pad
    ($pad:ty, $module:ty) => {
        impl Pin for $pad {
            type Direction = RX;
            type Module = $module;

            _rx_config!();
        }
    };
}

macro_rules! _impl_tx {
    // Implement a TX pad that needs daisy configuration
    ($pad:ty, $module:ty, $daisy_reg:ident, $daisy_val:ident) => {
        impl Pin for $pad {
            type Direction = TX;
            type Module = $module;

            _tx_config!($daisy_reg, $daisy_val);
        }
    };
    // Implement a TX pad
    ($pad:ty, $module:ty) => {
        impl Pin for $pad {
            type Direction = TX;
            type Module = $module;

            _tx_config!();
        }
    };
}

macro_rules! uart {
    // UART TX and RX pins, both which need a daisy selection
    ($module:ty, tx: $tx_pad:ty, $tx_daisy_reg:ident, $tx_daisy_val:ident, rx: $rx_pad:ty, $rx_daisy_reg:ident, $rx_daisy_val:ident,) => {
        _impl_tx!($tx_pad, $module, $tx_daisy_reg, $tx_daisy_val);
        _impl_rx!($rx_pad, $module, $rx_daisy_reg, $rx_daisy_val);
    };
    // UART TX and RX pins
    ($module:ty, tx: $tx_pad:ty, rx: $rx_pad:ty,) => {
        _impl_tx!($tx_pad, $module);
        _impl_rx!($rx_pad, $module);
    };
}

uart! {
    module::_6,
    tx: GPIO_AD_B0_02<Alt2>, lpuart6_tx_select_input, gpio_ad_b0_02_alt2,
    rx: GPIO_AD_B0_03<Alt2>, lpuart6_rx_select_input, gpio_ad_b0_03_alt2,
}

uart! {
    module::_1,
    tx: GPIO_AD_B0_12<Alt2>,
    rx: GPIO_AD_B0_13<Alt2>,
}

uart! {
    module::_3,
    tx: GPIO_AD_B1_06<Alt2>, lpuart3_tx_select_input, gpio_ad_b1_06_alt2,
    rx: GPIO_AD_B1_07<Alt2>, lpuart3_rx_select_input, gpio_ad_b1_07_alt2,
}

uart! {
    module::_4,
    tx: GPIO_B1_00<Alt2>, lpuart4_tx_select_input, gpio_b1_00_alt2,
    rx: GPIO_B1_01<Alt2>, lpuart4_rx_select_input, gpio_b1_01_alt2,
}

uart! {
    module::_2,
    tx: GPIO_AD_B1_02<Alt2>, lpuart2_tx_select_input, gpio_ad_b1_02_alt2,
    rx: GPIO_AD_B1_03<Alt2>, lpuart2_rx_select_input, gpio_ad_b1_03_alt2,
}

uart! {
    module::_7,
    tx: GPIO_EMC_31<Alt2>, lpuart7_tx_select_input, gpio_emc_31_alt2,
    rx: GPIO_EMC_32<Alt2>, lpuart7_rx_select_input, gpio_emc_32_alt2,
}

uart! {
    module::_8,
    tx: GPIO_AD_B1_10<Alt2>, lpuart8_tx_select_input, gpio_ad_b1_10_alt2,
    rx: GPIO_AD_B1_11<Alt2>, lpuart8_rx_select_input, gpio_ad_b1_11_alt2,
}

//! I2C pin multiplexing

pub mod module {
    pub trait Module {
        const IDX: usize;
    }
    pub struct _1;
    impl Module for _1 {
        const IDX: usize = 1;
    }
    pub struct _2;
    impl Module for _2 {
        const IDX: usize = 2;
    }
    pub struct _3;
    impl Module for _3 {
        const IDX: usize = 3;
    }
    pub struct _4;
    impl Module for _4 {
        const IDX: usize = 4;
    }
}

pub trait Wire {}
pub struct SCL;
impl Wire for SCL {}
pub struct SDA;
impl Wire for SDA {}

pub trait Pin {
    type Wire: Wire;
    type Module: module::Module;

    /// Perform IOMUXC configurations for this pin
    fn configure(&mut self);
}

use crate::iomuxc::{
    daisy::{Daisy, IntoDaisy},
    gpio::{
        GPIO_AD_B0_12, GPIO_AD_B0_13, GPIO_AD_B1_00, GPIO_AD_B1_01, GPIO_AD_B1_06, GPIO_AD_B1_07,
        GPIO_SD_B0_00, GPIO_SD_B0_01,
    },
    Alt0, Alt1, Alt2, Alt3,
};

macro_rules! pin_config {
    ($w:expr) => {
        $w.ode()
            .ode_1_open_drain_enabled()
            .sre()
            .sre_1_fast_slew_rate()
            .dse()
            .dse_4_r0_4()
            .speed()
            .speed_2_medium_100mhz()
            .pke()
            .pke_1_pull_keeper_enabled()
            .pue()
            .pue_1_pull()
            .pus()
            .pus_3_22k_ohm_pull_up()
    };
}

impl Pin for GPIO_AD_B1_07<Alt1> {
    type Wire = SCL;
    type Module = module::_3;
    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_AD_B1_07<Alt1> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpi2c3_scl_select_input
            .write(|w| w.daisy().gpio_ad_b1_07_alt1());
        Daisy::new(self)
    }
}

impl Pin for GPIO_AD_B1_06<Alt1> {
    type Wire = SDA;
    type Module = module::_3;
    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_AD_B1_06<Alt1> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpi2c3_sda_select_input
            .write(|w| w.daisy().gpio_ad_b1_06_alt1());
        Daisy::new(self)
    }
}

impl Pin for GPIO_AD_B1_01<Alt3> {
    type Wire = SDA;
    type Module = module::_1;
    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_AD_B1_01<Alt3> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpi2c1_sda_select_input
            .write(|w| w.daisy().gpio_ad_b1_01_alt3());
        Daisy::new(self)
    }
}

impl Pin for GPIO_AD_B1_00<Alt3> {
    type Wire = SCL;
    type Module = module::_1;
    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_AD_B1_00<Alt3> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpi2c1_scl_select_input
            .write(|w| w.daisy().gpio_ad_b1_00_alt3());
        Daisy::new(self)
    }
}

impl Pin for GPIO_AD_B0_12<Alt0> {
    type Wire = SCL;
    type Module = module::_4;
    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_AD_B0_12<Alt0> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpi2c4_scl_select_input
            .write(|w| w.daisy().gpio_ad_b0_12_alt0());
        Daisy::new(self)
    }
}

impl Pin for GPIO_AD_B0_13<Alt0> {
    type Wire = SDA;
    type Module = module::_4;
    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_AD_B0_13<Alt0> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpi2c4_sda_select_input
            .write(|w| w.daisy().gpio_ad_b0_13_alt0());
        Daisy::new(self)
    }
}

impl Pin for GPIO_SD_B0_01<Alt2> {
    type Wire = SDA;
    type Module = module::_3;
    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_SD_B0_01<Alt2> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpi2c3_sda_select_input
            .write(|w| w.daisy().gpio_sd_b0_01_alt2());
        Daisy::new(self)
    }
}

impl Pin for GPIO_SD_B0_00<Alt2> {
    type Wire = SCL;
    type Module = module::_3;
    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_SD_B0_00<Alt2> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpi2c3_scl_select_input
            .write(|w| w.daisy().gpio_sd_b0_00_alt2());
        Daisy::new(self)
    }
}

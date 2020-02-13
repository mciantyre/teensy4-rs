//! SPI pin multiplexing

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
pub struct SCK;
impl Wire for SCK {}
pub struct SDO;
impl Wire for SDO {}
pub struct SDI;
impl Wire for SDI {}
pub struct PCS0;
impl Wire for PCS0 {}

pub trait Pin {
    type Wire: Wire;
    type Module: module::Module;

    /// Perform IOMUXC configurations for this pin
    fn configure(&mut self);
}

use crate::iomuxc::{
    daisy::{Daisy, IntoDaisy},
    gpio::{
        GPIO_AD_B0_00, GPIO_AD_B0_01, GPIO_AD_B0_02, GPIO_AD_B0_03, GPIO_B0_00, GPIO_B0_01,
        GPIO_B0_02, GPIO_B0_03, GPIO_EMC_30, GPIO_SD_B0_00, GPIO_SD_B0_01, GPIO_SD_B0_02,
        GPIO_SD_B0_03, GPIO_SD_B1_06, GPIO_SD_B1_07, GPIO_SD_B1_08, GPIO_SD_B1_09,
    },
    Alt3, Alt4, Alt7,
};

macro_rules! pin_config {
    ($w:expr) => {
        $w.dse().dse_7_r0_7().speed().speed_2_medium_100mhz()
    };
}

//
// Note: The pad mapping below is *not* complete
//

// SPI 1

impl Pin for GPIO_SD_B0_00<Alt4> {
    type Wire = SCK;
    type Module = module::_1;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_SD_B0_00<Alt4> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi1_sck_select_input
            .write(|w| w.daisy().gpio_sd_b0_00_alt4());
        Daisy::new(self)
    }
}

impl Pin for GPIO_SD_B0_02<Alt4> {
    type Wire = SDO;
    type Module = module::_1;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_SD_B0_02<Alt4> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi1_sdo_select_input
            .write(|w| w.daisy().gpio_sd_b0_02_alt4());
        Daisy::new(self)
    }
}

impl Pin for GPIO_SD_B0_03<Alt4> {
    type Wire = SDI;
    type Module = module::_1;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_SD_B0_03<Alt4> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi1_sdi_select_input
            .write(|w| w.daisy().gpio_sd_b0_03_alt4());
        Daisy::new(self)
    }
}

impl Pin for GPIO_SD_B0_01<Alt4> {
    type Wire = PCS0;
    type Module = module::_1;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_SD_B0_01<Alt4> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi1_pcs0_select_input
            .write(|w| w.daisy().gpio_sd_b0_01_alt4());
        Daisy::new(self)
    }
}

impl Pin for GPIO_EMC_30<Alt3> {
    type Wire = PCS0;
    type Module = module::_1;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_EMC_30<Alt3> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi1_pcs0_select_input
            .write(|w| w.daisy().gpio_emc_30_alt3());
        Daisy::new(self)
    }
}

// SPI 2

impl Pin for GPIO_SD_B1_07<Alt4> {
    type Wire = SCK;
    type Module = module::_2;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_SD_B1_07<Alt4> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi2_sck_select_input
            .write(|w| w.daisy().gpio_sd_b1_07_alt4());
        Daisy::new(self)
    }
}

impl Pin for GPIO_SD_B1_08<Alt4> {
    type Wire = SDO;
    type Module = module::_2;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_SD_B1_08<Alt4> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi2_sdo_select_input
            .write(|w| w.daisy().gpio_sd_b1_08_alt4());
        Daisy::new(self)
    }
}

impl Pin for GPIO_SD_B1_09<Alt4> {
    type Wire = SDI;
    type Module = module::_2;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_SD_B1_09<Alt4> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi2_sdi_select_input
            .write(|w| w.daisy().gpio_sd_b1_09_alt4());
        Daisy::new(self)
    }
}

impl Pin for GPIO_SD_B1_06<Alt4> {
    type Wire = PCS0;
    type Module = module::_2;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_SD_B1_06<Alt4> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi2_pcs0_select_input
            .write(|w| w.daisy().gpio_sd_b1_06_alt4());
        Daisy::new(self)
    }
}

// SPI 3

impl Pin for GPIO_AD_B0_00<Alt7> {
    type Wire = SCK;
    type Module = module::_3;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_AD_B0_00<Alt7> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi3_sck_select_input
            .write(|w| w.daisy().gpio_ad_b0_00_alt7());
        Daisy::new(self)
    }
}

impl Pin for GPIO_AD_B0_01<Alt7> {
    type Wire = SDO;
    type Module = module::_3;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_AD_B0_01<Alt7> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi3_sdo_select_input
            .write(|w| w.daisy().gpio_ad_b0_01_alt7());
        Daisy::new(self)
    }
}

impl Pin for GPIO_AD_B0_02<Alt7> {
    type Wire = SDI;
    type Module = module::_3;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_AD_B0_02<Alt7> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi3_sdi_select_input
            .write(|w| w.daisy().gpio_ad_b0_02_alt7());
        Daisy::new(self)
    }
}

impl Pin for GPIO_AD_B0_03<Alt7> {
    type Wire = PCS0;
    type Module = module::_3;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_AD_B0_03<Alt7> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi3_pcs0_select_input
            .write(|w| w.daisy().gpio_ad_b0_03_alt7());
        Daisy::new(self)
    }
}

// SPI 4

impl Pin for GPIO_B0_03<Alt3> {
    type Wire = SCK;
    type Module = module::_4;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_B0_03<Alt3> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi4_sck_select_input
            .write(|w| w.daisy().gpio_b0_03_alt3());
        Daisy::new(self)
    }
}

impl Pin for GPIO_B0_02<Alt3> {
    type Wire = SDO;
    type Module = module::_4;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_B0_02<Alt3> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi4_sdo_select_input
            .write(|w| w.daisy().gpio_b0_02_alt3());
        Daisy::new(self)
    }
}

impl Pin for GPIO_B0_01<Alt3> {
    type Wire = SDI;
    type Module = module::_4;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_B0_01<Alt3> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi4_sdi_select_input
            .write(|w| w.daisy().gpio_b0_01_alt3());
        Daisy::new(self)
    }
}

impl Pin for GPIO_B0_00<Alt3> {
    type Wire = PCS0;
    type Module = module::_4;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        self.pad().write(|w| pin_config!(w));
    }
}

impl IntoDaisy for GPIO_B0_00<Alt3> {
    #[inline(always)]
    fn into_daisy(self) -> Daisy<Self> {
        self.iomuxc()
            .lpspi4_pcs0_select_input
            .write(|w| w.daisy().gpio_b0_00_alt3());
        Daisy::new(self)
    }
}

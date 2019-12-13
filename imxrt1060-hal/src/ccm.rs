//! Clock Configuration Module (CCM)

use imxrt1060_pac as pac;

pub struct Handle {
    pub(crate) base: pac::CCM,
    pub(crate) analog: pac::CCM_ANALOG,
}

impl Handle {
    pub fn raw(self) -> (pac::CCM, pac::CCM_ANALOG) {
        (self.base, self.analog)
    }
}

pub struct CCM {
    pub handle: Handle,
    pub perclk: perclk::Multiplexer,
    /// The 480 MHz PFD
    pub pll2: pll2::PFD,
    /// The 528 MHz PFD
    pub pll3: pll3::PFD,
}

impl CCM {
    pub(crate) fn new(base: pac::CCM, analog: pac::CCM_ANALOG) -> Self {
        CCM {
            handle: Handle { base, analog },
            perclk: perclk::Multiplexer::new(),
            pll2: pll2::PFD::new(),
            pll3: pll3::PFD::new(),
        }
    }
}

pub mod perclk {
    use super::pac;
    use super::Handle;

    pub type PODF = pac::ccm::cscmr1::PERCLK_PODF_A;
    pub type CLKSEL = pac::ccm::cscmr1::PERCLK_CLK_SEL_A;

    pub struct Multiplexer;
    pub struct Configured<'a>(pub(crate) &'a mut Handle);

    impl Multiplexer {
        pub(super) fn new() -> Self {
            Multiplexer
        }

        pub fn configure(self, h: &mut Handle, podf: PODF, clksel: CLKSEL) -> Configured {
            h.base.cscmr1.modify(|_, w| {
                w.perclk_podf()
                    .variant(podf)
                    .perclk_clk_sel()
                    .variant(clksel)
            });
            Configured(h)
        }
    }
}

macro_rules! pfd {
    ($setter:ident, $value:ident) => {
        use super::Handle;

        pub struct PFD;
        impl PFD {
            pub(super) fn new() -> Self {
                PFD
            }

            #[inline(always)]
            pub fn set(self, handle: &mut Handle, pfds: [Option<Frequency>; 4]) {
                handle.analog.$setter.write(|w| {
                    w.pfd0_clkgate()
                        .bit(pfds[0].is_some())
                        .pfd1_clkgate()
                        .bit(pfds[1].is_some())
                        .pfd2_clkgate()
                        .bit(pfds[2].is_some())
                        .pfd3_clkgate()
                        .bit(pfds[3].is_some())
                });

                handle.analog.$value.write(|w| unsafe {
                    if let Some(bits) = &pfds[0] {
                        w.pfd0_frac().bits(bits.0);
                    }
                    if let Some(bits) = &pfds[1] {
                        w.pfd1_frac().bits(bits.0);
                    }
                    if let Some(bits) = &pfds[2] {
                        w.pfd2_frac().bits(bits.0);
                    }
                    if let Some(bits) = &pfds[3] {
                        w.pfd3_frac().bits(bits.0);
                    }
                    w
                });
            }
        }
    };
}

/// 480 MHz phase fractional divider
pub mod pll3 {
    pfd!(pfd_480_set, pfd_480);

    pub struct Frequency(u8);

    pub const MHZ_720: Frequency = Frequency(12);
    pub const MHZ_664: Frequency = Frequency(13);
    pub const MHZ_617: Frequency = Frequency(14);
    pub const MHZ_576: Frequency = Frequency(15);
    pub const MHZ_540: Frequency = Frequency(16);
    pub const MHZ_508: Frequency = Frequency(17);
    pub const MHZ_480: Frequency = Frequency(18);
    pub const MHZ_454: Frequency = Frequency(19);
    pub const MHZ_432: Frequency = Frequency(20);
    pub const MHZ_411: Frequency = Frequency(21);
    pub const MHZ_392: Frequency = Frequency(22);
    pub const MHZ_375: Frequency = Frequency(23);
    pub const MHZ_360: Frequency = Frequency(24);
    pub const MHZ_345: Frequency = Frequency(25);
    pub const MHZ_332: Frequency = Frequency(26);
    pub const MHZ_320: Frequency = Frequency(27);
    pub const MHZ_308: Frequency = Frequency(28);
    pub const MHZ_297: Frequency = Frequency(29);
    pub const MHZ_288: Frequency = Frequency(30);
    pub const MHZ_278: Frequency = Frequency(31);
    pub const MHZ_270: Frequency = Frequency(32);
    pub const MHZ_261: Frequency = Frequency(33);
    pub const MHZ_254: Frequency = Frequency(34);
    pub const MHZ_246: Frequency = Frequency(35);
}

/// 528 MHz phase fractional divider
pub mod pll2 {
    pfd!(pfd_528_set, pfd_528);
    pub struct Frequency(u8);

    pub const MHZ_792: Frequency = Frequency(12);
    pub const MHZ_731: Frequency = Frequency(13);
    pub const MHZ_678: Frequency = Frequency(14);
    pub const MHZ_633: Frequency = Frequency(15);
    pub const MHZ_594: Frequency = Frequency(16);
    pub const MHZ_559: Frequency = Frequency(17);
    pub const MHZ_528: Frequency = Frequency(18);
    pub const MHZ_500: Frequency = Frequency(19);
    pub const MHZ_475: Frequency = Frequency(20);
    pub const MHZ_452: Frequency = Frequency(21);
    pub const MHZ_432: Frequency = Frequency(22);
    pub const MHZ_413: Frequency = Frequency(23);
    pub const MHZ_396: Frequency = Frequency(24);
    pub const MHZ_380: Frequency = Frequency(25);
    pub const MHZ_365: Frequency = Frequency(26);
    pub const MHZ_352: Frequency = Frequency(27);
    pub const MHZ_339: Frequency = Frequency(28);
    pub const MHZ_327: Frequency = Frequency(29);
    pub const MHZ_316: Frequency = Frequency(30);
    pub const MHZ_306: Frequency = Frequency(31);
    pub const MHZ_297: Frequency = Frequency(32);
    pub const MHZ_288: Frequency = Frequency(33);
    pub const MHZ_279: Frequency = Frequency(34);
    pub const MHZ_271: Frequency = Frequency(35);
}

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
}

impl CCM {
    pub(crate) fn new(base: pac::CCM, analog: pac::CCM_ANALOG) -> Self {
        CCM {
            handle: Handle { base, analog },
            perclk: perclk::Multiplexer::new(),
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

        #[inline(always)] // TODO why does this need to be inlined?
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

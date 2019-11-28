//! Periodic Interrupt Timer (PIT)

use crate::ccm::perclk;
use core::marker::PhantomData;
use imxrt1060_pac as pac;

pub trait Timer {
    fn timer() -> &'static pac::pit::TIMER;
}

pub struct T1;
impl Timer for T1 {
    fn timer() -> &'static pac::pit::TIMER {
        unsafe { &(*pac::PIT::ptr()).timer[1] }
    }
}

pub struct Unclocked;
pub struct Clocked;

pub struct PIT<State> {
    _s: PhantomData<State>,
    base: pac::PIT,
}

impl<State> PIT<State> {
    #[inline(always)]
    pub(crate) fn new(base: pac::PIT) -> PIT<State> {
        PIT {
            _s: PhantomData,
            base,
        }
    }
}

impl PIT<Unclocked> {
    #[inline(always)]
    pub fn clock(self, configured: perclk::Configured) -> PIT<Clocked> {
        configured
            .0
            .base
            .ccgr1
            .modify(|_, w| unsafe { w.cg6().bits(0x3) });
        self.base.mcr.write(|w| w.mdis().mdis_0());
        PIT::new(self.base)
    }
}

impl PIT<Clocked> {
    #[inline(always)]
    pub fn timer<T: Timer>(&mut self) -> (Control, Rearm) {
        (Control(T::timer()), Rearm(T::timer()))
    }
}

pub struct Control(&'static pac::pit::TIMER);

impl Control {
    #[inline(always)]
    pub fn load(&mut self, tsv: u32) -> &mut Self {
        self.0.ldval.write(|w| unsafe { w.tsv().bits(tsv) });
        self
    }

    #[inline(always)]
    pub fn enable(&mut self) -> &mut Self {
        // Need to be written in two separate writes...?
        self.0.tctrl.write(|w| w.tie().tie_1());
        self.0.tctrl.modify(|_, w| w.ten().ten_1());
        self
    }
}

pub struct Rearm(&'static pac::pit::TIMER);
impl Rearm {
    #[inline(always)]
    pub fn rearm(&mut self) {
        self.0.tflg.write(|w| w.tif().tif_1())
    }
}

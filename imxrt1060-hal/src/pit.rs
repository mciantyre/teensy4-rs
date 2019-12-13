//! Periodic Interrupt Timer (PIT)

use crate::ccm::perclk;
use embedded_hal::timer::{CountDown, Periodic};
use imxrt1060_pac as pac;

/// An unclocked periodic interrupt timer module
///
/// In order to activate the PIT, we must pass in the
/// configured object returned from the CCM's perclk module.
pub struct UnclockedPIT(pac::PIT);

impl UnclockedPIT {
    pub(crate) fn new(base: pac::PIT) -> Self {
        UnclockedPIT(base)
    }

    /// Activate the PIT module after enabling the clock for the
    /// module.
    pub fn clock(self, configured: perclk::Configured) -> [PIT; 4] {
        configured
            .0
            .base
            .ccgr1
            .modify(|_, w| unsafe { w.cg6().bits(0x3) });
        self.0.mcr.write(|w| w.mdis().mdis_0());
        [PIT::new(0), PIT::new(1), PIT::new(2), PIT::new(3)]
    }
}

/// A periodic interrupt timer (PIT)
pub struct PIT(&'static pac::pit::TIMER);

impl PIT {
    fn new(idx: usize) -> PIT {
        unsafe { PIT(&(*pac::PIT::ptr()).timer[idx]) }
    }
}

/// A milliseconds period used for PIT time keeping
#[derive(Clone, Copy)]
pub struct Milliseconds(u32);

pub trait U32Ext {
    fn ms(self) -> Milliseconds;
}

impl U32Ext for u32 {
    fn ms(self) -> Milliseconds {
        Milliseconds(self)
    }
}

impl CountDown for PIT {
    type Time = Milliseconds;
    fn start<T: Into<Self::Time>>(&mut self, count: T) {
        self.0.tctrl.write(|w| w.ten().clear_bit());
        self.0
            .ldval
            .write(|w| unsafe { w.tsv().bits(count.into().0) });
        self.0.tctrl.write(|w| w.ten().set_bit());
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        if self.0.tflg.read().tif().bit_is_set() {
            self.0.tflg.write(|w| w.tif().set_bit());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl Periodic for PIT {}

//! Periodic Interrupt Timer (PIT)

use crate::ccm::{perclk, ticks, Divider, Frequency, Ticks, TicksError};
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
        let (clock_hz, divider) = configured.enable();
        self.0.mcr.write(|w| w.mdis().mdis_0());
        [
            PIT::new(0, clock_hz, divider),
            PIT::new(1, clock_hz, divider),
            PIT::new(2, clock_hz, divider),
            PIT::new(3, clock_hz, divider),
        ]
    }
}

/// A periodic interrupt timer (PIT)
pub struct PIT {
    timer: &'static pac::pit::TIMER,
    clock_hz: Frequency,
    divider: Divider,
}

impl PIT {
    fn new(idx: usize, clock_hz: Frequency, divider: Divider) -> PIT {
        PIT {
            // Safety: register is static; index is within half-closed range [0,4)
            // in `UnclockedPIT::clock()`
            timer: unsafe { &(*pac::PIT::ptr()).timer[idx] },
            clock_hz,
            divider,
        }
    }
}

impl CountDown for PIT {
    type Time = core::time::Duration;
    fn start<T: Into<Self::Time>>(&mut self, ms: T) {
        let ticks = match ticks(ms.into(), self.clock_hz, self.divider) {
            Ok(ticks) => ticks,
            // Saturate the load value
            Err(TicksError::TicksOverflow) | Err(TicksError::DurationOverflow) => {
                Ticks(core::u32::MAX)
            }
            // Ratio of freq / div was zero, or divider was zero
            Err(TicksError::DivideByZero) => Ticks(1),
        };
        self.timer.tctrl.write(|w| w.ten().clear_bit());
        // Safety: TSV register is 32 bits wide
        self.timer.ldval.write(|w| unsafe { w.tsv().bits(ticks.0) });
        self.timer.tctrl.write(|w| w.ten().set_bit());
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        if self.timer.tflg.read().tif().bit_is_set() {
            // Timer complete. W1C...
            self.timer.tflg.write(|w| w.tif().set_bit());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl Periodic for PIT {}

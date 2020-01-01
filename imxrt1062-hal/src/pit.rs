//! Periodic Interrupt Timer (PIT)

use crate::ccm::{perclk, ticks, Divider, Frequency, Ticks, TicksError};
use core::marker::PhantomData;
use embedded_hal::timer::{CountDown, Periodic};
use imxrt1062_pac as pac;

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
    pub fn clock(
        self,
        configured: perclk::Configured,
    ) -> (
        PIT<channel::_0>,
        PIT<channel::_1>,
        PIT<channel::_2>,
        PIT<channel::_3>,
    ) {
        let (clock_hz, divider) = configured.enable();
        self.0.mcr.write(|w| w.mdis().mdis_0());
        (
            PIT::new(clock_hz, divider),
            PIT::new(clock_hz, divider),
            PIT::new(clock_hz, divider),
            PIT::new(clock_hz, divider),
        )
    }
}

pub mod channel {
    /// Dummy channel for describing channel chaining.
    ///
    /// Timer 0 cannot be chained. This is the only "valid" chainable
    /// channel, but it does not exist.
    pub struct _X;
    /// PIT channel 0
    pub struct _0;
    /// PIT channel 1
    pub struct _1;
    /// PIT channel 2
    pub struct _2;
    /// PIT channel 3
    pub struct _3;

    pub trait Channel {
        const IDX: usize;
        type ChainedTo: Channel;
    }

    impl Channel for _X {
        const IDX: usize = core::usize::MAX;
        type ChainedTo = _X;
    }

    impl Channel for _0 {
        const IDX: usize = 0;
        type ChainedTo = _X;
    }
    impl Channel for _1 {
        const IDX: usize = 1;
        type ChainedTo = _0;
    }
    impl Channel for _2 {
        const IDX: usize = 2;
        type ChainedTo = _1;
    }
    impl Channel for _3 {
        const IDX: usize = 3;
        type ChainedTo = _2;
    }
}

/// A periodic interrupt timer (PIT)
pub struct PIT<Chan> {
    timer: &'static pac::pit::TIMER,
    clock_hz: Frequency,
    divider: Divider,
    _chan: PhantomData<Chan>,
}

impl<Chan: channel::Channel> PIT<Chan> {
    fn new(clock_hz: Frequency, divider: Divider) -> PIT<Chan> {
        PIT {
            // Safety: register is static; index is within half-closed range [0,4)
            // in `UnclockedPIT::clock()`
            timer: unsafe { &(*pac::PIT::ptr()).timer[Chan::IDX] },
            clock_hz,
            divider,
            _chan: PhantomData,
        }
    }

    fn disabled<F: FnMut()>(&self, mut act: F) {
        self.timer.tctrl.reset();
        act();
        self.timer.tctrl.modify(|_, w| w.ten().set_bit());
    }

    fn ldval(&self, val: u32) {
        // Safety: TSV register is 32 bits wide
        self.timer.ldval.write(|w| unsafe { w.tsv().bits(val) });
    }

    fn tif(&self) -> bool {
        self.timer.tflg.read().tif().bit_is_set()
    }

    fn clear_tif(&mut self) {
        // W1C
        self.timer.tflg.write(|w| w.tif().set_bit());
    }
}

impl<Chan: channel::Channel> CountDown for PIT<Chan> {
    type Time = core::time::Duration;
    fn start<T: Into<Self::Time>>(&mut self, ms: T) {
        let ticks: Ticks<u32> = match ticks(ms.into(), self.clock_hz, self.divider) {
            Ok(ticks) => ticks,
            // Saturate the load value
            Err(TicksError::TicksOverflow) | Err(TicksError::DurationOverflow) => {
                Ticks(core::u32::MAX)
            }
            // Ratio of freq / div was zero, or divider was zero
            Err(TicksError::DivideByZero) => Ticks(0),
        };
        self.disabled(|| {
            self.ldval(ticks.0);
        });
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        if self.tif() {
            self.clear_tif();
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<Chan: channel::Channel> Periodic for PIT<Chan> {}

/// Two PIT timers chained together
pub struct ChainedPIT<C0, C1> {
    lower: PIT<C0>,
    upper: PIT<C1>,
}

/// Chain two timers together, returning a `ChainedPIT` timer that can
/// count twice as many ticks.
///
/// The API enforces that channel 1 is chained to channel 0, or channel 2 is
/// chained to channel 1, or channel 3 is chained to channel 2. Any other
/// combination of chaining is prevented by the compiler.
///
/// We do not support chaining more than two timers.
pub fn chain<C1: channel::Channel>(
    lower: PIT<<C1 as channel::Channel>::ChainedTo>,
    upper: PIT<C1>,
) -> ChainedPIT<<C1 as channel::Channel>::ChainedTo, C1> {
    ChainedPIT { lower, upper }
}

impl<C0, C1> CountDown for ChainedPIT<C0, C1>
where
    C0: channel::Channel,
    C1: channel::Channel,
{
    type Time = core::time::Duration;
    fn start<T: Into<Self::Time>>(&mut self, time: T) {
        // clock_hz and divider are equal across all PITs
        let ticks: Ticks<u64> = match ticks(time.into(), self.lower.clock_hz, self.lower.divider) {
            Ok(ticks) => ticks,
            // Saturate the load value
            Err(TicksError::TicksOverflow) | Err(TicksError::DurationOverflow) => {
                Ticks(core::u64::MAX)
            }
            // Ratio of freq / div was zero, or divider was zero
            Err(TicksError::DivideByZero) => Ticks(0),
        };
        self.lower.disabled(|| {
            self.upper.disabled(|| {
                self.upper.timer.tctrl.modify(|_, w| w.chn().set_bit());
                self.upper.ldval((ticks.0 >> 32) as u32);
                self.lower.ldval((ticks.0 & 0xFFFF_FFFF) as u32)
            })
        });
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        if self.lower.tif() && self.upper.tif() {
            self.lower.clear_tif();
            self.upper.clear_tif();
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

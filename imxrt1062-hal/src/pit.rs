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

    #[inline(always)]
    fn disabled<F: FnMut(&Self) -> R, R>(&self, mut act: F) -> R {
        let enabled = self.timer.tctrl.read().ten().bit_is_set();
        self.timer.tctrl.modify(|_, w| w.ten().clear_bit());
        let tsv = self.timer.ldval.read().tsv().bits();
        let res = act(self);
        self.ldval(tsv);
        self.timer.tctrl.modify(|_, w| w.ten().bit(enabled));
        res
    }

    fn ldval(&self, val: u32) {
        // Safety: TSV register is 32 bits wide
        self.timer.ldval.write(|w| unsafe { w.tsv().bits(val) });
    }

    fn tif(&self) -> bool {
        self.timer.tflg.read().tif().bit_is_set()
    }

    fn clear_tif(&self) {
        // W1C
        self.timer.tflg.write(|w| w.tif().set_bit());
    }

    /// Returns the period of the clock ticks. This is the inverse
    /// of the clock frequency
    pub fn clock_period(&self) -> core::time::Duration {
        (self.clock_hz / self.divider).into()
    }

    /// Measure the execution duration of `act` with this timer. Returns the duration
    /// of the action, or `None` if the timer expired before the action completed.
    ///
    /// `time` will measure the difference of counts in a 32 bit register. The counter
    /// changes every clock period. The clock accuracy is based on our ability to round
    /// integers. Consider choosing the input clock frequency and prescalars to define
    /// a clock that can accurately measure your workloads.
    ///
    /// The method will disable any interrupts that this timer has enabled. It will also
    /// reset the timer to execute this measurement.
    ///
    /// If you need a 64 bit timer, use the `chain` function to combine timer 0 and
    /// timer 1. The two can crate the 'lifetime' timer, which is capable of measuring
    /// larger intervals.
    pub fn time<F: FnMut() -> R, R>(&mut self, mut act: F) -> (R, Option<core::time::Duration>) {
        const STARTING_LDVAL: u32 = u32::max_value();
        self.with_interrupts_disabled(|this| {
            this.disabled(|this| {
                this.clear_tif();
                this.ldval(STARTING_LDVAL);
                self.timer.tctrl.modify(|_, w| w.ten().set_bit());
                let res = act();
                let counter = this.timer.cval.read().tvl().bits();
                if this.tif() {
                    // Action took too long and the timer expired.
                    // The counter value is meaningless
                    (res, None)
                } else {
                    let ticks = STARTING_LDVAL - counter;
                    let clock_period: core::time::Duration = (this.clock_hz / this.divider).into();
                    (res, Some(ticks * clock_period))
                }
            })
        })
    }

    /// Enable the timer to trigger an interrupt when the timer expires
    pub fn set_interrupt_enable(&mut self, interrupt: bool) {
        self.timer.tctrl.modify(|_, w| w.tie().bit(interrupt));
    }

    /// Returns `true` if the timer will trigger an interrupt when
    /// it expires.
    pub fn interrupt_enable(&self) -> bool {
        self.timer.tctrl.read().tie().bit_is_set()
    }

    #[inline(always)]
    fn with_interrupts_disabled<F: FnMut(&Self) -> R, R>(&self, mut act: F) -> R {
        let interrupt_enabled = self.interrupt_enable();
        self.timer.tctrl.modify(|_, w| w.tie().clear_bit());
        let res = act(self);
        self.timer
            .tctrl
            .modify(|_, w| w.tie().bit(interrupt_enabled));
        res
    }
}

impl<Chan: channel::Channel> CountDown for PIT<Chan> {
    type Time = core::time::Duration;
    fn start<T: Into<Self::Time>>(&mut self, ms: T) {
        let ticks: Ticks<u32> = match ticks(ms.into(), self.clock_hz, self.divider) {
            Ok(ticks) => ticks,
            // Saturate the load value
            Err(TicksError::TicksOverflow) | Err(TicksError::DurationOverflow) => {
                Ticks(u32::max_value())
            }
            // Ratio of freq / div was zero, or divider was zero
            Err(TicksError::DivideByZero) => Ticks(0),
        };
        self.timer.tctrl.modify(|_, w| w.ten().clear_bit());
        self.clear_tif();
        self.ldval(ticks.0);
        self.timer.tctrl.modify(|_, w| w.ten().set_bit());
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

impl<C0, C1> ChainedPIT<C0, C1>
where
    C1: channel::Channel,
{
    /// Control interrupt generation for this chained PIT timer
    pub fn set_interrupt_enable(&mut self, interrupt: bool) {
        self.upper.set_interrupt_enable(interrupt);
    }

    /// Returns `true` if interrupts are enabled, else `false`
    /// if interrupts are disabled.
    pub fn interrupt_enable(&self) -> bool {
        self.upper.interrupt_enable()
    }
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
    mut lower: PIT<<C1 as channel::Channel>::ChainedTo>,
    upper: PIT<C1>,
) -> ChainedPIT<<C1 as channel::Channel>::ChainedTo, C1> {
    // We can only enable the interrupt for the upper timer.
    // Otherwise, we'll interrupt early.
    lower.set_interrupt_enable(false);
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
                Ticks(u64::max_value())
            }
            // Ratio of freq / div was zero, or divider was zero
            Err(TicksError::DivideByZero) => Ticks(0),
        };
        self.lower.timer.tctrl.modify(|_, w| w.ten().clear_bit());
        self.upper.timer.tctrl.modify(|_, w| w.ten().clear_bit());

        self.upper.clear_tif();
        self.upper.timer.tctrl.modify(|_, w| w.chn().set_bit());
        self.upper.ldval((ticks.0 >> 32) as u32);
        self.lower.ldval((ticks.0 & 0xFFFF_FFFF) as u32);

        self.lower.timer.tctrl.modify(|_, w| w.ten().set_bit());
        self.upper.timer.tctrl.modify(|_, w| w.ten().set_bit());
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        if self.upper.tif() {
            self.upper.clear_tif();
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

/// The lifetime timer is PIT0 chained to PIT1.
/// It allows us to time over 64 bits with no
/// carry.
impl ChainedPIT<channel::_0, channel::_1> {
    /// Time the execution duration of `act`. Returns the time it took to run `act`,
    /// or `None` if the timer expired.
    ///
    /// See the notes on `PIT::time`. Unlike `PIT::time`, this `time` method uses
    /// a 64 bit register, which can help measure larger intervals. As with `PIT::time`,
    /// this function will temporarily disable interrupts and reset any currently-running
    /// timer.
    ///
    /// This method is only available when chaining timer 0 to timer 1.
    pub fn time<F: FnMut() -> R, R>(&mut self, mut act: F) -> (R, Option<core::time::Duration>) {
        const STARTING_LDVAL: u32 = u32::max_value();
        self.upper.with_interrupts_disabled(|upper| {
            self.lower.disabled(|lower| {
                upper.disabled(|upper| {
                    upper.clear_tif();
                    upper.ldval(STARTING_LDVAL);
                    lower.ldval(STARTING_LDVAL);

                    upper.timer.tctrl.modify(|_, w| w.chn().set_bit());
                    upper.timer.tctrl.modify(|_, w| w.ten().set_bit());
                    lower.timer.tctrl.modify(|_, w| w.ten().set_bit());

                    let res = act();
                    let lifetime = {
                        let pit = unsafe { &*pac::PIT::ptr() };

                        let lifetime: u64 = (pit.ltmr64h.read().bits() as u64) << 32;
                        lifetime | (pit.ltmr64l.read().bits() as u64)
                    };
                    if upper.tif() {
                        (res, None)
                    } else {
                        let ticks = u64::max_value() - lifetime;
                        // Betting that this isn't lossy...
                        let clock_period: u64 =
                            core::time::Duration::from(upper.clock_hz / upper.divider).as_nanos()
                                as u64;
                        (
                            res,
                            Some(core::time::Duration::from_nanos(ticks * clock_period)),
                        )
                    }
                })
            })
        })
    }
}

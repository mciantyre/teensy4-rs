//! Pulse Width Modulation (PWM)
//!
//! The PWM module provides abstractions for using
//! the iMXRT1062's PWM capabilities. It relies on
//! a user mutliplexing appropriate pins from.
//! IOMUXC. It also relies on some of the timing
//! functionality available in the CCM. See those
//! modules for details.
//!
//! # Usage
//!
//! In the list below, we describe the typical usage of
//! the PWM abstractions. For example code, please consult
//! the examples in the `teensy4-examples` crate.
//!
//! 1. The system starts up with unclocked PWM controllers,
//! represented as `UnclockedController`. Enable clocking
//! to the controller by providing the CCM's handle. This
//! returns a `Controller` type that can be used to allocate
//! PWM pins. The example below shows how to enable clocks for the PWM2
//! module.
//!
//! 2. Obtain PWM pin pairs by providing processor pads to
//! the PWM controller. As of this writing, we only provide output
//! PWM pins, and complementary PWM pins (PWM23 and PWM45 in processor
//! parlance) are not implemented. Use `output()` to transform PWM pins
//! into output PWM pairs, then use `split()` to make the pins independent
//! instances. It's at this stage that we specify the PWM driving period
//! (frequency), the prescalar, and the clock selection. We may only
//! select the IPG clock; new variants will be added as necessary.
//!
//! 3. The two `PWM` instances returned from `split()` implement the
//! `embedded_hal::PwmPin` trait, and they may be used to control
//! duty cycles.

use core::marker::PhantomData;

use crate::ccm;
use crate::iomuxc::pwm::Pin;
pub use crate::iomuxc::pwm::{module, output, submodule};
use embedded_hal::PwmPin;
use imxrt1062_pac as pac;

/// A PWM module that is not receiving a clock input
///
/// An `UnclockedController` transitions to a `Controller`
/// by calling `clock()`.
pub struct UnclockedController<M> {
    _module: PhantomData<M>,
}

impl<M> UnclockedController<M>
where
    M: module::Module,
{
    pub(crate) fn new() -> Self {
        UnclockedController {
            _module: PhantomData,
        }
    }
}

macro_rules! clock_impl {
    ($module:path, $cg:ident, $pwm:ty) => {
        impl UnclockedController<$module> {
            /// Enable the input clock for this PWM module. Returns a PWM `Controller`
            /// that can allocated PWM outputs.
            pub fn clock(self, handle: &mut ccm::Handle) -> Controller<$module> {
                let (ccm, _) = handle.raw();
                // Safety: field is 2 bits
                ccm.ccgr4.modify(|_, w| unsafe { w.$cg().bits(0x3) });
                Controller::new(<$pwm>::ptr())
            }
        }
    };
}

clock_impl!(module::_1, cg8, pac::PWM1);
clock_impl!(module::_2, cg9, pac::PWM2);
clock_impl!(module::_3, cg10, pac::PWM3);
clock_impl!(module::_4, cg11, pac::PWM4);

#[derive(Clone, Copy)]
struct Reg(&'static pac::pwm1::RegisterBlock);
impl core::ops::Deref for Reg {
    type Target = pac::pwm1::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Reg {
    fn reset_ok<S, F, R>(&mut self, mut act: F) -> R
    where
        F: FnMut(&pac::pwm1::SM) -> R,
        S: submodule::Submodule,
    {
        let idx: usize = <S as submodule::Submodule>::IDX;
        self.0.mctrl.modify(|_, w| unsafe {
            // Safety, cldok is 4 bits, idx is bound [0, 4)
            w.cldok().bits(1 << idx)
        });
        let ret = act(<S as submodule::Submodule>::submodule(self.0));
        self.0.mctrl.modify(|_, w| unsafe {
            // Safety: ldok is 4 bits, idx is bound [0, 4)
            w.ldok().bits(1 << idx)
        });
        ret
    }
}

/// Specifies the timing-related parameters for a PWM submodule
#[derive(Clone, Copy)]
pub struct Timing {
    /// The clock selection for the PWM submodule
    pub clock_select: ccm::pwm::ClockSelect,
    /// The clock divider for the PWM submodule
    pub prescalar: ccm::pwm::Prescalar,
    /// The driving (switching) frequency, expressed as a period
    pub switching_period: core::time::Duration,
}

/// A PWM controller
///
/// There's one PWM controller per module. The controller allows a user
/// to allocated PWM outputs. It allows a user to perform module-level
/// configurations.
pub struct Controller<M> {
    reg: Reg,
    _module: PhantomData<M>,
}

impl<M> Controller<M>
where
    M: module::Module,
{
    fn new(reg: *const pac::pwm1::RegisterBlock) -> Self {
        let pwm = Controller {
            // Safety: pointer points to static memory
            reg: unsafe { Reg(&(*reg)) },
            _module: PhantomData,
        };

        pwm.reg.fctrl0.write_with_zero(|w| unsafe {
            // Safety: flvl is four bits
            w.flvl().bits(0xF)
        });
        pwm.reg.fsts0.write_with_zero(|w| unsafe {
            // Safety: fflag is four bits
            w.fflag().bits(0xF)
        });

        pwm
    }

    /// Allocates a `Pair` of complementary PWM output pins containing pins `A` and `B`.
    ///
    /// `pin_a` and `pin_b` may be obtained from the IOMUXC. Pins may need
    /// to be set to a certain alternative in order to be passed into the
    /// `outputs` method. The two pins must be associated with this controller's
    /// module, and the submodules of each pin must match.
    ///
    /// Specify the timings for the PWM module / submodule with the `Timing`
    /// parameter. If the provided timings cannot define a representable
    /// driving frequency, `outputs` returns an error.
    pub fn outputs<A, B>(
        &mut self,
        pin_a: A,
        pin_b: B,
        timing: Timing,
    ) -> Result<Pairs<M, <A as Pin>::Submodule>, ccm::TicksError>
    where
        A: Pin<Module = M, Output = output::A>,
        B: Pin<Module = M, Output = output::B, Submodule = <A as Pin>::Submodule>,
    {
        let idx = <<A as Pin>::Submodule as submodule::Submodule>::IDX;
        self.reg.reset_ok::<<A as Pin>::Submodule, _, _>(|sm| {
            sm.smctrl2.write(|w| {
                w.waiten()
                    .set_bit()
                    .dbgen()
                    .set_bit()
                    .clk_sel()
                    .variant(timing.clock_select.into())
            });
            sm.smctrl
                .write(|w| w.full().set_bit().prsc().variant(timing.prescalar));

            sm.smoctrl.write_with_zero(|w| w);
            sm.smdtcnt0.write_with_zero(|w| w);

            sm.sminit.write_with_zero(|w| w);
            sm.smval0.reset();
            let ticks: ccm::Ticks<u16> = ccm::ticks(
                timing.switching_period,
                timing.clock_select.into(),
                timing.prescalar.into(),
            )?;
            sm.smval1.write(|w| unsafe {
                // Safety: val1 is 16 bits
                w.val1().bits(ticks.0)
            });
            sm.smval2.reset();
            sm.smval3.reset();
            sm.smval4.reset();
            sm.smval5.reset();
            Ok(())
        })?;
        self.reg.mctrl.modify(|r, w| unsafe {
            let mask = (1 << idx) & 0xF;
            // Safety: four bits
            w.run().bits(mask | r.run().bits())
        });
        Ok(Pairs::new(self.reg, pin_a, pin_b))
    }
}

/// PWM complementary pairs
///
/// The struct doesn't have value right now. Use `split()` to turn
/// the two pins into independent PWM outputs.
pub struct Pairs<M, S> {
    pin_a: PWM<M, S, output::A>,
    pin_b: PWM<M, S, output::B>,
}

impl<M, S> Pairs<M, S>
where
    M: module::Module,
    S: submodule::Submodule,
{
    fn new<A, B>(reg: Reg, pin_a: A, pin_b: B) -> Self
    where
        A: Pin<Module = M, Submodule = S, Output = output::A>,
        B: Pin<Module = M, Submodule = S, Output = output::B>,
    {
        Pairs {
            pin_a: PWM::new(reg, pin_a),
            pin_b: PWM::new(reg, pin_b),
        }
    }

    /// Consumes the complementary pairs, and sets the two PWM outputs for
    /// use as independent outputs.
    pub fn split(mut self) -> (PWM<M, S, output::A>, PWM<M, S, output::B>) {
        self.pin_a.reg.reset_ok::<S, _, ()>(|sm| {
            sm.smctrl2.modify(|_, w| w.indep().set_bit());
        });
        (self.pin_a, self.pin_b)
    }
}

/// A PWM output instance. It has a module, submodule, and output type.
///
/// Implements the `embedded_hal::PwmPin` interface.
pub struct PWM<M, S, O> {
    reg: Reg,
    _module: PhantomData<M>,
    _submodule: PhantomData<S>,
    _output: PhantomData<O>,
}

impl<M, S, O> PWM<M, S, O>
where
    M: module::Module,
    S: submodule::Submodule,
    O: output::Output,
{
    fn new<P>(reg: Reg, _pin: P) -> Self
    where
        P: Pin<Module = M, Submodule = S, Output = O>,
    {
        PWM {
            reg,
            _module: PhantomData,
            _submodule: PhantomData,
            _output: PhantomData,
        }
    }
}

impl<M, S> PwmPin for PWM<M, S, output::A>
where
    M: module::Module,
    S: submodule::Submodule,
{
    type Duty = u16;

    fn disable(&mut self) {
        self.reg.outen.modify(|r, w| unsafe {
            let idx = <S as submodule::Submodule>::IDX;
            let mask = !(1 << idx) & 0xF;
            // Safety: each bits() is 4 bits
            w.pwma_en().bits(mask & r.pwma_en().bits())
        });
    }

    fn enable(&mut self) {
        self.reg.outen.modify(|r, w| unsafe {
            let idx = <S as submodule::Submodule>::IDX;
            let mask = (1 << idx) & 0xF;
            // Safety: each bits() is 4 bits
            w.pwma_en().bits(mask | r.pwma_en().bits())
        });
    }

    fn get_duty(&self) -> Self::Duty {
        let sm = <S as submodule::Submodule>::submodule(self.reg.0);
        let modulo: u32 = sm.smval1.read().bits() as u32;
        let cval: u32 = sm.smval3.read().bits() as u32;
        ((cval << 16) / (modulo + 1)) as u16
    }

    fn get_max_duty(&self) -> Self::Duty {
        core::u16::MAX
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        self.reg.reset_ok::<S, _, ()>(|sm| {
            let modulo: u32 = sm.smval1.read().bits() as u32;
            let cval = ((duty as u32) * (modulo + 1)) >> 16;
            let cval = if cval > modulo {
                modulo as u16
            } else {
                cval as u16
            };
            sm.smval3.write(|w| unsafe {
                // Safety: val3 is 16 bits
                w.val3().bits(cval)
            });
        });
    }
}

impl<M, S> PwmPin for PWM<M, S, output::B>
where
    M: module::Module,
    S: submodule::Submodule,
{
    type Duty = u16;

    fn disable(&mut self) {
        self.reg.outen.modify(|r, w| unsafe {
            let idx = <S as submodule::Submodule>::IDX;
            let mask = !(1 << idx) & 0xF;
            // Safety: each bits() is 4 bits
            w.pwmb_en().bits(mask & r.pwmb_en().bits())
        });
    }

    fn enable(&mut self) {
        self.reg.outen.modify(|r, w| unsafe {
            let idx = <S as submodule::Submodule>::IDX;
            let mask = (1 << idx) & 0xF;
            // Safety: each bits() is 4 bits
            w.pwmb_en().bits(mask | r.pwmb_en().bits())
        });
    }

    fn get_duty(&self) -> Self::Duty {
        let sm = <S as submodule::Submodule>::submodule(self.reg.0);
        let modulo: u32 = sm.smval1.read().bits() as u32;
        let cval: u32 = sm.smval5.read().bits() as u32;
        ((cval << 16) / (modulo + 1)) as u16
    }

    fn get_max_duty(&self) -> Self::Duty {
        core::u16::MAX
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        self.reg.reset_ok::<S, _, ()>(|sm| {
            let modulo: u32 = sm.smval1.read().bits() as u32;
            let cval = ((duty as u32) * (modulo + 1)) >> 16;
            let cval = if cval > modulo {
                modulo as u16
            } else {
                cval as u16
            };
            sm.smval5.write(|w| unsafe {
                // Safety: val5 is 16 bits
                w.val5().bits(cval)
            });
        });
    }
}

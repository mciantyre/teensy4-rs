//! Pulse Width Modulation (PWM)

use core::marker::PhantomData;

use crate::ccm;
use crate::iomuxc::pwm::Pin;
pub use crate::iomuxc::pwm::{module, output, submodule};
use embedded_hal::PwmPin;
use imxrt1060_pac as pac;

pub struct UnclockedPWMController<M> {
    _module: PhantomData<M>,
}

impl<M> UnclockedPWMController<M>
where
    M: module::Module,
{
    pub(crate) fn new() -> Self {
        UnclockedPWMController {
            _module: PhantomData,
        }
    }
}

macro_rules! clock_impl {
    ($module:path, $cg:ident, $pwm:ty) => {
        impl UnclockedPWMController<$module> {
            pub fn clock(self, handle: &mut ccm::Handle) -> PWMController<$module> {
                let (ccm, _) = handle.raw();
                // Safety: field is 2 bits
                ccm.ccgr4.write(|w| unsafe { w.$cg().bits(0x3) });
                PWMController::new(<$pwm>::ptr())
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

pub struct PWMController<M> {
    reg: Reg,
    _module: PhantomData<M>,
}

impl<M> PWMController<M>
where
    M: module::Module,
{
    fn new(reg: *const pac::pwm1::RegisterBlock) -> Self {
        let pwm = PWMController {
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

    pub fn outputs<A, B>(&mut self, pin_a: A, pin_b: B) -> PWMPairs<M, <A as Pin>::Submodule>
    where
        A: Pin<Module = M, Output = output::A>,
        B: Pin<Module = M, Output = output::B, Submodule = <A as Pin>::Submodule>,
    {
        self.reg.outen.modify(|r, w| unsafe {
            // Safety: each bits() is 4 bits
            let idx = <<A as Pin>::Submodule as submodule::Submodule>::IDX;
            w.pwma_en()
                .bits((1 << idx) | r.pwma_en().bits())
                .pwmb_en()
                .bits((1 << idx) | r.pwmb_en().bits())
        });
        self.reg.reset_ok::<<A as Pin>::Submodule, _, ()>(|sm| {
            sm.smctrl2.write(|w| w.waiten().set_bit().dbgen().set_bit());
            sm.smctrl.write(|w| w.full().set_bit());

            sm.smoctrl.write_with_zero(|w| w);
            sm.smdtcnt0.write_with_zero(|w| w);
            sm.sminit.write_with_zero(|w| w);

            sm.smval0.reset();
            // Safety: val1 is 16 bits
            sm.smval1.write(|w| unsafe { w.val1().bits(10_000) });
            sm.smval2.reset();
            sm.smval3.reset();
            sm.smval4.reset();
            sm.smval5.reset();
        });
        PWMPairs::new(self.reg, pin_a, pin_b)
    }
}

pub struct PWMPairs<M, S> {
    pin_a: PWM<M, S, output::A>,
    pin_b: PWM<M, S, output::B>,
}

impl<M, S> PWMPairs<M, S>
where
    M: module::Module,
    S: submodule::Submodule,
{
    fn new<A, B>(reg: Reg, pin_a: A, pin_b: B) -> Self
    where
        A: Pin<Module = M, Submodule = S, Output = output::A>,
        B: Pin<Module = M, Submodule = S, Output = output::B>,
    {
        PWMPairs {
            pin_a: PWM::new(reg, pin_a),
            pin_b: PWM::new(reg, pin_b),
        }
    }

    pub fn split(mut self) -> (PWM<M, S, output::A>, PWM<M, S, output::B>) {
        self.pin_a.reg.reset_ok::<S, _, ()>(|sm| {
            sm.smctrl2.modify(|_, w| w.indep().set_bit());
        });
        (self.pin_a, self.pin_b)
    }
}

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
        let idx = <S as submodule::Submodule>::IDX;
        let mask = !(1 << idx) & 0xF;
        self.reg.mctrl.modify(|r, w| unsafe {
            // Safety: four bits
            w.run().bits(mask & r.run().bits())
        });
    }

    fn enable(&mut self) {
        let idx = <S as submodule::Submodule>::IDX;
        let mask = (1 << idx) & 0xF;
        self.reg.mctrl.modify(|r, w| unsafe {
            // Safety: four bits
            w.run().bits(mask | r.run().bits())
        });
    }

    fn get_duty(&self) -> Self::Duty {
        // TODO not sure if that's right...
        let sm = <S as submodule::Submodule>::submodule(self.reg.0);
        sm.smval3.read().bits()
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
        let idx = <S as submodule::Submodule>::IDX;
        let mask = !(1 << idx) & 0xF;
        self.reg.mctrl.modify(|r, w| unsafe {
            // Safety: four bits
            w.run().bits(mask & r.run().bits())
        });
    }

    fn enable(&mut self) {
        let idx = <S as submodule::Submodule>::IDX;
        let mask = (1 << idx) & 0xF;
        self.reg.mctrl.modify(|r, w| unsafe {
            // Safety: four bits
            w.run().bits(mask | r.run().bits())
        });
    }

    fn get_duty(&self) -> Self::Duty {
        // TODO not sure if that's right...
        let sm = <S as submodule::Submodule>::submodule(self.reg.0);
        sm.smval5.read().bits()
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

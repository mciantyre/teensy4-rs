//! Pulse Width Modulation (PWM)

use core::marker::PhantomData;

use crate::ccm;
use crate::iomuxc::pwm::Pin;
pub use crate::iomuxc::pwm::{module, output, submodule};
use imxrt1060_pac as pac;

pub struct UnclockedPWMController<M> {
    _module: PhantomData<M>,
}

macro_rules! clock_impl {
    ($module:path, $cg:ident, $pwm:ty) => {
        impl UnclockedPWMController<$module> {
            pub fn clock(
                self,
                ipg_hz: ccm::IPGFrequency,
                handle: &mut ccm::Handle,
            ) -> PWMController<$module> {
                let (ccm, _) = handle.raw();
                // Safety: field is 2 bits
                ccm.ccgr4.write(|w| unsafe { w.$cg().bits(0x3) });
                PWMController::new(ipg_hz, <$pwm>::ptr())
            }
        }
    };
}

clock_impl!(module::_1, cg8, pac::PWM1);
clock_impl!(module::_2, cg9, pac::PWM2);
clock_impl!(module::_3, cg10, pac::PWM3);
clock_impl!(module::_4, cg11, pac::PWM4);

pub struct PWMController<M> {
    ipg_hz: ccm::IPGFrequency,
    _module: PhantomData<M>,
    reg: &'static pac::pwm1::RegisterBlock,
}

impl<M> PWMController<M>
where
    M: module::Module,
{
    fn new(ipg_hz: ccm::IPGFrequency, reg: *const pac::pwm1::RegisterBlock) -> Self {
        PWMController {
            ipg_hz,
            _module: PhantomData,
            reg: unsafe { &(*reg) },
        }
    }

    pub fn output<P: Pin<Module = M>>(&mut self, pin: P) -> PWM<M, <P as Pin>::Submodule> {
        // This is an interesting transition from a compile time to a runtime property.
        // TODO see if this is applicable elsewhere.
        PWM {
            ipg_hz: self.ipg_hz,
            reg: self.reg,
            _module_submodule: PhantomData,
        }
    }
}

pub struct PWM<M, S> {
    ipg_hz: ccm::IPGFrequency,
    reg: &'static pac::pwm1::RegisterBlock,
    _module_submodule: PhantomData<(M, S)>,
}

macro_rules! pwm_impl {
    ($idx:path) => {
        impl<M> PWM<M, $idx>
        where
            M: module::Module,
        {
            fn new(ipg_hz: ccm::IPGFrequency, reg: &'static pac::pwm1::RegisterBlock) -> Self {
                reg.fctrl0.modify(|r, w| unsafe {
                    w.flvl()
                        .bits(1 << (12 + <$idx as submodule::Submodule>::IDX) | r.flvl().bits())
                });
                PWM {
                    ipg_hz,
                    reg,
                    _module_submodule: PhantomData,
                }
            }
        }
    };
}

pwm_impl!(submodule::_2);

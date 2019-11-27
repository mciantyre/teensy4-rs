use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin, ToggleableOutputPin};

/// Denotes that a pin is configured as an input
pub struct Input;
/// Denotes that a pin is configured as an output
pub struct Output;

pub struct GPIO2;
pub struct GPIO7;

#[doc(hidden)]
pub trait IntoRegister {
    fn into_reg() -> *const crate::pac::gpio1::RegisterBlock;
}

impl IntoRegister for GPIO2 {
    fn into_reg() -> *const crate::pac::gpio1::RegisterBlock {
        crate::pac::GPIO2::ptr()
    }
}

impl IntoRegister for GPIO7 {
    fn into_reg() -> *const crate::pac::gpio1::RegisterBlock {
        crate::pac::GPIO7::ptr()
    }
}

macro_rules! _ios_impl {
    ($($io:ident)+) => {
        $(
            pub struct $io<GPIO, Dir> {
                _gpio: core::marker::PhantomData<GPIO>,
                _dir: core::marker::PhantomData<Dir>,
                offset: u32,
            }

            impl<GPIO: IntoRegister> $io<GPIO, Input> {
                pub fn output(self) -> $io<GPIO, Output> {
                    unsafe { (*GPIO::into_reg()).gdir.modify(|r, w| w.bits(self.offset | r.bits()))};
                    $io{ _gpio: core::marker::PhantomData, _dir: core::marker::PhantomData, offset: self.offset }
                }
            }

            impl<GPIO: IntoRegister> OutputPin for $io<GPIO, Output> {
                type Error = core::convert::Infallible; // '!' Not available on stable

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    unsafe { (*GPIO::into_reg()).dr_clear.write(|reg| reg.bits(self.offset)) };
                    Ok(())
                }

                fn set_high(&mut self) -> Result<(), Self::Error> {
                    unsafe { (*GPIO::into_reg()).dr_set.write(|reg| reg.bits(self.offset)) };
                    Ok(())
                }
            }

            impl<GPIO: IntoRegister> StatefulOutputPin for $io<GPIO, Output> {
                fn is_set_high(&self) -> Result<bool, Self::Error> {
                    Ok(unsafe { (*GPIO::into_reg()).psr.read().bits() & self.offset > 0 })
                }

                fn is_set_low(&self) -> Result<bool, Self::Error> {
                    self.is_set_high().map(|res| !res)
                }
            }

            impl<GPIO: IntoRegister> ToggleableOutputPin for $io<GPIO, Output> {
                type Error = core::convert::Infallible;
                fn toggle(&mut self) -> Result<(), Self::Error> {
                    unsafe { (*GPIO::into_reg()).dr_toggle.write(|reg| reg.bits(self.offset)) };
                    Ok(())
                }
            }

        )+
    };
}

macro_rules! _ios_ctor {
    ($($n:expr, $io:ident, $bank:ty, $ctor:ident, $pad:ty)+) => {
        $(
            impl $io<$bank, Input> {
                pub fn $ctor(_pad: $pad) -> Self {
                    Self {
                        _gpio: core::marker::PhantomData,
                        _dir: core::marker::PhantomData,
                        offset: 1 << $n,
                    }
                }
            }
        )+
    };
}

macro_rules! _ios_fast {
    ($($io:ident, $slow:ty, $fast:ty, $gprfn:ident)+) => {
        $(
            impl $io<$slow, Input> {
                pub fn fast(self, gpr: &mut $crate::iomuxc::GPR) -> $io<$fast, Input> {
                    let reg = gpr.$gprfn();
                    reg.modify(|r, w| unsafe { w.bits(self.offset | r.bits()) });
                    $io {
                        _gpio: core::marker::PhantomData,
                        _dir: core::marker::PhantomData,
                        offset: self.offset
                    }
                }
            }
        )+
    };
}

macro_rules! ios {
    ($($n:expr, $io:ident: [$bank:ty, $ctor:ident, $pad:ty],)+) => {
        _ios_impl!($($io)+);
        _ios_ctor!($($n, $io, $bank, $ctor, $pad)+);
    };
    ($($n:expr, $io:ident: [$bank:ty, $ctor:ident, $pad:ty, FAST: ($fast:ty, $gprfn:ident)],)+) => {
        _ios_impl!($($io)+);
        _ios_ctor!($($n, $io, $bank, $ctor, $pad)+);
        _ios_fast!($($io, $bank, $fast, $gprfn)+);
    };
}

ios! {
    3, IO03: [GPIO2, gpio2, crate::iomuxc::gpio::GPIO_B0_03<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
}

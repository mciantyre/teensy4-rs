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

            impl<GPIO: IntoRegister> $io<GPIO, Output> {
                /// Toggle the internal state of the pin
                pub fn toggle(&mut self) {
                    unsafe { (*GPIO::into_reg()).dr_toggle.write(|reg| reg.bits(self.offset)) }
                }

                /// Drive the pin logically low. Note: this may not be electrically low
                pub fn low(&mut self) {
                    unsafe { (*GPIO::into_reg()).dr_clear.write(|reg| reg.bits(self.offset)) }
                }

                /// Drive the pin to logical high. Note: this may not be electrically high
                pub fn high(&mut self) {
                    unsafe { (*GPIO::into_reg()).dr_set.write(|reg| reg.bits(self.offset)) }
                }

                /// Returns `true` if logically high, else `false` if logically low
                pub fn state(&self) -> bool {
                    unsafe { (*GPIO::into_reg()).psr.read().bits() & self.offset > 0 }
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

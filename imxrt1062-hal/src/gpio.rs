use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};

/// Denotes that a pin is configured as an input
pub struct Input;
/// Denotes that a pin is configured as an output
pub struct Output;

pub struct GPIO1;
pub struct GPIO2;
pub struct GPIO3;
pub struct GPIO4;
pub struct GPIO5;
pub struct GPIO6;
pub struct GPIO7;

pub trait IntoGpio {
    type Gpio;
    fn into_gpio(self) -> Self::Gpio;
}

#[doc(hidden)]
pub trait IntoRegister {
    fn into_reg() -> *const crate::pac::gpio1::RegisterBlock;
}

impl IntoRegister for GPIO1 {
    fn into_reg() -> *const crate::pac::gpio1::RegisterBlock {
        crate::pac::GPIO1::ptr()
    }
}

impl IntoRegister for GPIO2 {
    fn into_reg() -> *const crate::pac::gpio1::RegisterBlock {
        crate::pac::GPIO2::ptr()
    }
}

impl IntoRegister for GPIO3 {
    fn into_reg() -> *const crate::pac::gpio1::RegisterBlock {
        crate::pac::GPIO3::ptr()
    }
}

impl IntoRegister for GPIO4 {
    fn into_reg() -> *const crate::pac::gpio1::RegisterBlock {
        crate::pac::GPIO4::ptr()
    }
}

impl IntoRegister for GPIO5 {
    fn into_reg() -> *const crate::pac::gpio1::RegisterBlock {
        crate::pac::GPIO5::ptr()
    }
}

impl IntoRegister for GPIO6 {
    fn into_reg() -> *const crate::pac::gpio1::RegisterBlock {
        crate::pac::GPIO6::ptr()
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

            impl<GPIO: IntoRegister> InputPin for $io<GPIO, Input> {
                type Error = core::convert::Infallible; // '!' Not available on stable

                fn is_low(&self) -> Result<bool, Self::Error> {
                    Ok(unsafe { (*GPIO::into_reg()).psr.read().psr().bits() & self.offset == 0 })
                }

                fn is_high(&self) -> Result<bool, Self::Error> {
                    Ok(unsafe { (*GPIO::into_reg()).psr.read().psr().bits() & self.offset != 0 })
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

            impl From<$pad> for $io<$bank, Input> {
                fn from(pad: $pad) -> Self {
                    Self::$ctor(pad)
                }
            }

            impl IntoGpio for $pad {
                type Gpio = $io<$bank, Input>;
                fn into_gpio(self) -> Self::Gpio {
                    $io::<$bank, Input>::$ctor(self)
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

// Bank 1; these are really controlled by IOMUXC_GPR_GPR26 but for some reason I can't access that
// register here.
ios! {
    0, GPIO1IO00: [GPIO1, gpio0, crate::iomuxc::gpio::GPIO_AD_B0_00<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    1, GPIO1IO01: [GPIO1, gpio1, crate::iomuxc::gpio::GPIO_AD_B0_01<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    2, GPIO1IO02: [GPIO1, gpio2, crate::iomuxc::gpio::GPIO_AD_B0_02<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    3, GPIO1IO03: [GPIO1, gpio3, crate::iomuxc::gpio::GPIO_AD_B0_03<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    4, GPIO1IO04: [GPIO1, gpio4, crate::iomuxc::gpio::GPIO_AD_B0_04<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    5, GPIO1IO05: [GPIO1, gpio5, crate::iomuxc::gpio::GPIO_AD_B0_05<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    6, GPIO1IO06: [GPIO1, gpio6, crate::iomuxc::gpio::GPIO_AD_B0_06<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    7, GPIO1IO07: [GPIO1, gpio7, crate::iomuxc::gpio::GPIO_AD_B0_07<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    8, GPIO1IO08: [GPIO1, gpio8, crate::iomuxc::gpio::GPIO_AD_B0_08<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    9, GPIO1IO09: [GPIO1, gpio9, crate::iomuxc::gpio::GPIO_AD_B0_09<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    10, GPIO1IO10: [GPIO1, gpio10, crate::iomuxc::gpio::GPIO_AD_B0_10<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    11, GPIO1IO11: [GPIO1, gpio11, crate::iomuxc::gpio::GPIO_AD_B0_11<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    12, GPIO1IO12: [GPIO1, gpio12, crate::iomuxc::gpio::GPIO_AD_B0_12<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    13, GPIO1IO13: [GPIO1, gpio13, crate::iomuxc::gpio::GPIO_AD_B0_13<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    14, GPIO1IO14: [GPIO1, gpio14, crate::iomuxc::gpio::GPIO_AD_B0_14<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    15, GPIO1IO15: [GPIO1, gpio15, crate::iomuxc::gpio::GPIO_AD_B0_15<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    16, GPIO1IO16: [GPIO1, gpio16, crate::iomuxc::gpio::GPIO_AD_B1_00<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    17, GPIO1IO17: [GPIO1, gpio17, crate::iomuxc::gpio::GPIO_AD_B1_01<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    18, GPIO1IO18: [GPIO1, gpio18, crate::iomuxc::gpio::GPIO_AD_B1_02<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    19, GPIO1IO19: [GPIO1, gpio19, crate::iomuxc::gpio::GPIO_AD_B1_03<crate::iomuxc::Alt5>, FAST: (GPIO6, gpr26)],
    // TODO: more from this bank? Missing 20..=31
}

// Bank 2
ios! {
    0, GPIO2IO00: [GPIO2, gpio0, crate::iomuxc::gpio::GPIO_B0_00<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    1, GPIO2IO01: [GPIO2, gpio1, crate::iomuxc::gpio::GPIO_B0_01<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    2, GPIO2IO02: [GPIO2, gpio2, crate::iomuxc::gpio::GPIO_B0_02<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    3, GPIO2IO03: [GPIO2, gpio3, crate::iomuxc::gpio::GPIO_B0_03<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    4, GPIO2IO04: [GPIO2, gpio4, crate::iomuxc::gpio::GPIO_B0_04<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    5, GPIO2IO05: [GPIO2, gpio5, crate::iomuxc::gpio::GPIO_B0_05<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    6, GPIO2IO06: [GPIO2, gpio6, crate::iomuxc::gpio::GPIO_B0_06<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    7, GPIO2IO07: [GPIO2, gpio7, crate::iomuxc::gpio::GPIO_B0_07<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    8, GPIO2IO08: [GPIO2, gpio8, crate::iomuxc::gpio::GPIO_B0_08<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    9, GPIO2IO09: [GPIO2, gpio9, crate::iomuxc::gpio::GPIO_B0_09<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    10, GPIO2IO10: [GPIO2, gpio10, crate::iomuxc::gpio::GPIO_B0_10<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    11, GPIO2IO11: [GPIO2, gpio11, crate::iomuxc::gpio::GPIO_B0_11<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    12, GPIO2IO12: [GPIO2, gpio12, crate::iomuxc::gpio::GPIO_B0_12<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    13, GPIO2IO13: [GPIO2, gpio13, crate::iomuxc::gpio::GPIO_B0_13<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    14, GPIO2IO14: [GPIO2, gpio14, crate::iomuxc::gpio::GPIO_B0_14<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    15, GPIO2IO15: [GPIO2, gpio15, crate::iomuxc::gpio::GPIO_B0_15<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    16, GPIO2IO16: [GPIO2, gpio16, crate::iomuxc::gpio::GPIO_B1_00<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    17, GPIO2IO17: [GPIO2, gpio17, crate::iomuxc::gpio::GPIO_B1_01<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    18, GPIO2IO18: [GPIO2, gpio18, crate::iomuxc::gpio::GPIO_B1_02<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    19, GPIO2IO19: [GPIO2, gpio19, crate::iomuxc::gpio::GPIO_B1_03<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    20, GPIO2IO20: [GPIO2, gpio20, crate::iomuxc::gpio::GPIO_B1_04<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    21, GPIO2IO21: [GPIO2, gpio21, crate::iomuxc::gpio::GPIO_B1_05<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    22, GPIO2IO22: [GPIO2, gpio22, crate::iomuxc::gpio::GPIO_B1_06<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    23, GPIO2IO23: [GPIO2, gpio23, crate::iomuxc::gpio::GPIO_B1_07<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    24, GPIO2IO24: [GPIO2, gpio24, crate::iomuxc::gpio::GPIO_B1_08<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    25, GPIO2IO25: [GPIO2, gpio25, crate::iomuxc::gpio::GPIO_B1_09<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    26, GPIO2IO26: [GPIO2, gpio26, crate::iomuxc::gpio::GPIO_B1_10<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    27, GPIO2IO27: [GPIO2, gpio27, crate::iomuxc::gpio::GPIO_B1_11<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    28, GPIO2IO28: [GPIO2, gpio28, crate::iomuxc::gpio::GPIO_B1_12<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    29, GPIO2IO29: [GPIO2, gpio29, crate::iomuxc::gpio::GPIO_B1_13<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    30, GPIO2IO30: [GPIO2, gpio30, crate::iomuxc::gpio::GPIO_B1_14<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
    31, GPIO2IO31: [GPIO2, gpio31, crate::iomuxc::gpio::GPIO_B1_15<crate::iomuxc::Alt5>, FAST: (GPIO7, gpr27)],
}

// Bank 3, missing quite a few here...
ios! {
    // gap 0..=5
    6, GPIO3IO06: [GPIO3, gpio6, crate::iomuxc::gpio::GPIO_SD_B1_06<crate::iomuxc::Alt5>],
    7, GPIO3IO07: [GPIO3, gpio7, crate::iomuxc::gpio::GPIO_SD_B1_07<crate::iomuxc::Alt5>],
    8, GPIO3IO08: [GPIO3, gpio8, crate::iomuxc::gpio::GPIO_SD_B1_08<crate::iomuxc::Alt5>],
    9, GPIO3IO09: [GPIO3, gpio9, crate::iomuxc::gpio::GPIO_SD_B1_09<crate::iomuxc::Alt5>],
    // gap 10..=11
    12, GPIO3IO12: [GPIO3, gpio12, crate::iomuxc::gpio::GPIO_SD_B0_00<crate::iomuxc::Alt5>],
    13, GPIO3IO13: [GPIO3, gpio13, crate::iomuxc::gpio::GPIO_SD_B0_01<crate::iomuxc::Alt5>],
    14, GPIO3IO14: [GPIO3, gpio14, crate::iomuxc::gpio::GPIO_SD_B0_02<crate::iomuxc::Alt5>],
    15, GPIO3IO15: [GPIO3, gpio15, crate::iomuxc::gpio::GPIO_SD_B0_03<crate::iomuxc::Alt5>],
    // gap 16..=17
    18, GPIO3IO18: [GPIO3, gpio18, crate::iomuxc::gpio::GPIO_EMC_32<crate::iomuxc::Alt5>],
    // gap 19..=27
}

// Bank 4, missing quite a few here...
ios! {
    // gap 0..=3
    4, GPIO4IO04: [GPIO4, gpio4, crate::iomuxc::gpio::GPIO_EMC_04<crate::iomuxc::Alt5>],
    5, GPIO4IO05: [GPIO4, gpio5, crate::iomuxc::gpio::GPIO_EMC_05<crate::iomuxc::Alt5>],
    6, GPIO4IO06: [GPIO4, gpio6, crate::iomuxc::gpio::GPIO_EMC_06<crate::iomuxc::Alt5>],
    // gap 7
    8, GPIO4IO08: [GPIO4, gpio8, crate::iomuxc::gpio::GPIO_EMC_08<crate::iomuxc::Alt5>],
    // gap 9..=29
    30, GPIO4IO30: [GPIO4, gpio30, crate::iomuxc::gpio::GPIO_EMC_30<crate::iomuxc::Alt5>],
    31, GPIO4IO31: [GPIO4, gpio31, crate::iomuxc::gpio::GPIO_EMC_31<crate::iomuxc::Alt5>],
}

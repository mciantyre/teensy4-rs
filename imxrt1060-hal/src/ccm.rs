//! Clock Configuration Module (CCM)

use core::time::Duration;
use imxrt1060_pac as pac;

pub struct Handle {
    pub(crate) base: pac::CCM,
    pub(crate) analog: pac::CCM_ANALOG,
}

impl Handle {
    pub fn raw(self) -> (pac::CCM, pac::CCM_ANALOG) {
        (self.base, self.analog)
    }
}

pub struct CCM {
    pub handle: Handle,
    pub perclk: perclk::Multiplexer,
    /// The 480 MHz PFD
    pub pll2: pll2::PFD,
    /// The 528 MHz PFD
    pub pll3: pll3::PFD,
}

impl CCM {
    pub(crate) fn new(base: pac::CCM, analog: pac::CCM_ANALOG) -> Self {
        CCM {
            handle: Handle { base, analog },
            perclk: perclk::Multiplexer::new(),
            pll2: pll2::PFD::new(),
            pll3: pll3::PFD::new(),
        }
    }
}

pub mod perclk {
    use super::{pac, Divider, Frequency, Handle};

    pub type PODF = pac::ccm::cscmr1::PERCLK_PODF_A;
    pub type CLKSEL = pac::ccm::cscmr1::PERCLK_CLK_SEL_A;

    pub struct Multiplexer;
    pub struct Configured<'a> {
        handle: &'a mut Handle,
        divider: Divider,
        clock_hz: Frequency,
    }

    impl Multiplexer {
        pub(super) fn new() -> Self {
            Multiplexer
        }

        pub fn configure(self, handle: &mut Handle, podf: PODF, clksel: CLKSEL) -> Configured {
            handle.base.cscmr1.modify(|_, w| {
                w.perclk_podf()
                    .variant(podf)
                    .perclk_clk_sel()
                    .variant(clksel)
            });
            Configured {
                handle,
                divider: Divider::from(podf),
                clock_hz: Frequency::from(clksel),
            }
        }
    }

    impl<'a> Configured<'a> {
        pub(crate) fn enable(self) -> (Frequency, Divider) {
            self.handle
                .base
                .ccgr1
                // Safety: CG6 is two bits wide
                .modify(|_, w| unsafe { w.cg6().bits(0x3) });
            (self.clock_hz, self.divider)
        }
    }

    impl From<CLKSEL> for Frequency {
        fn from(clksel: CLKSEL) -> Frequency {
            match clksel {
                // 24MHz oscillator
                CLKSEL::PERCLK_CLK_SEL_1 => Frequency(24_000_000),
                // TODO figure out IPG clock speed
                CLKSEL::PERCLK_CLK_SEL_0 => Frequency(1),
            }
        }
    }

    impl From<PODF> for Divider {
        fn from(podf: PODF) -> Divider {
            Divider((u8::from(podf) + 1) as u32)
        }
    }
}

macro_rules! pfd {
    ($setter:ident, $value:ident) => {
        use super::Handle;

        pub struct PFD;
        impl PFD {
            pub(super) fn new() -> Self {
                PFD
            }

            pub fn set(&mut self, handle: &mut Handle, pfds: [Option<Frequency>; 4]) {
                handle.analog.$setter.write(|w| {
                    w.pfd0_clkgate()
                        .bit(pfds[0].is_some())
                        .pfd1_clkgate()
                        .bit(pfds[1].is_some())
                        .pfd2_clkgate()
                        .bit(pfds[2].is_some())
                        .pfd3_clkgate()
                        .bit(pfds[3].is_some())
                });

                // Safety: PDFx_FRAC is 6 bits wide. By the implementations
                // of the `Frequency(..)` newtypes, the wrapped values will
                // never exceed a 6 bit value.
                handle.analog.$value.write(|w| unsafe {
                    if let Some(bits) = &pfds[0] {
                        w.pfd0_frac().bits(bits.0);
                    }
                    if let Some(bits) = &pfds[1] {
                        w.pfd1_frac().bits(bits.0);
                    }
                    if let Some(bits) = &pfds[2] {
                        w.pfd2_frac().bits(bits.0);
                    }
                    if let Some(bits) = &pfds[3] {
                        w.pfd3_frac().bits(bits.0);
                    }
                    w
                });
            }
        }
    };
}

/// 480 MHz phase fractional divider
pub mod pll3 {
    pfd!(pfd_480_set, pfd_480);

    pub struct Frequency(u8);

    pub const MHZ_720: Frequency = Frequency(12);
    pub const MHZ_664: Frequency = Frequency(13);
    pub const MHZ_617: Frequency = Frequency(14);
    pub const MHZ_576: Frequency = Frequency(15);
    pub const MHZ_540: Frequency = Frequency(16);
    pub const MHZ_508: Frequency = Frequency(17);
    pub const MHZ_480: Frequency = Frequency(18);
    pub const MHZ_454: Frequency = Frequency(19);
    pub const MHZ_432: Frequency = Frequency(20);
    pub const MHZ_411: Frequency = Frequency(21);
    pub const MHZ_392: Frequency = Frequency(22);
    pub const MHZ_375: Frequency = Frequency(23);
    pub const MHZ_360: Frequency = Frequency(24);
    pub const MHZ_345: Frequency = Frequency(25);
    pub const MHZ_332: Frequency = Frequency(26);
    pub const MHZ_320: Frequency = Frequency(27);
    pub const MHZ_308: Frequency = Frequency(28);
    pub const MHZ_297: Frequency = Frequency(29);
    pub const MHZ_288: Frequency = Frequency(30);
    pub const MHZ_278: Frequency = Frequency(31);
    pub const MHZ_270: Frequency = Frequency(32);
    pub const MHZ_261: Frequency = Frequency(33);
    pub const MHZ_254: Frequency = Frequency(34);
    pub const MHZ_246: Frequency = Frequency(35);
}

/// 528 MHz phase fractional divider
pub mod pll2 {
    pfd!(pfd_528_set, pfd_528);
    pub struct Frequency(u8);

    pub const MHZ_792: Frequency = Frequency(12);
    pub const MHZ_731: Frequency = Frequency(13);
    pub const MHZ_678: Frequency = Frequency(14);
    pub const MHZ_633: Frequency = Frequency(15);
    pub const MHZ_594: Frequency = Frequency(16);
    pub const MHZ_559: Frequency = Frequency(17);
    pub const MHZ_528: Frequency = Frequency(18);
    pub const MHZ_500: Frequency = Frequency(19);
    pub const MHZ_475: Frequency = Frequency(20);
    pub const MHZ_452: Frequency = Frequency(21);
    pub const MHZ_432: Frequency = Frequency(22);
    pub const MHZ_413: Frequency = Frequency(23);
    pub const MHZ_396: Frequency = Frequency(24);
    pub const MHZ_380: Frequency = Frequency(25);
    pub const MHZ_365: Frequency = Frequency(26);
    pub const MHZ_352: Frequency = Frequency(27);
    pub const MHZ_339: Frequency = Frequency(28);
    pub const MHZ_327: Frequency = Frequency(29);
    pub const MHZ_316: Frequency = Frequency(30);
    pub const MHZ_306: Frequency = Frequency(31);
    pub const MHZ_297: Frequency = Frequency(32);
    pub const MHZ_288: Frequency = Frequency(33);
    pub const MHZ_279: Frequency = Frequency(34);
    pub const MHZ_271: Frequency = Frequency(35);
}

use core::convert::TryFrom;
pub trait TicksRepr: TryFrom<u64> {}

impl TicksRepr for u32 {}
impl TicksRepr for u64 {}

/// An opaque duration representing the number of clock ticks
///
/// See the `ticks` function to derive a `Ticks` value.
#[derive(Clone, Copy)]
pub struct Ticks<R: TicksRepr>(pub(crate) R);

/// Possible errors that could result during a computation of `ticks`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TicksError {
    /// The duration cannot be expressed in a `u64`.
    DurationOverflow,
    /// The number of ticks cannot be expressed in a `u32`
    TicksOverflow,
    /// Computation would divide by zero
    DivideByZero,
}

/// Computes the number of clock ticks that span the provide duration, given
/// the clock frequency and clock divider. If there is no divider, use `Divider::default()`
/// to specify an unused divider. Returns `Ok(ticks)` when the computation of
/// clock ticks succeeds, or an error.
pub fn ticks<R: TicksRepr>(
    dur: Duration,
    freq: Frequency,
    div: Divider,
) -> Result<Ticks<R>, TicksError> {
    // Ticks computed as
    //
    //  ticks = (duration / clock_period) - 1
    //
    // where `clock_period` is the effective clock period: `freq / div`
    let delay_ns = u64::try_from(dur.as_nanos()).map_err(|_| TicksError::DurationOverflow)?;
    let effective_freq = freq
        .0
        .checked_div(div.0)
        .ok_or(TicksError::DurationOverflow)?;
    let clock_period_ns = 1_000_000_000u32
        .checked_div(effective_freq)
        .map(u64::from)
        .ok_or(TicksError::DivideByZero)?;
    delay_ns
        .checked_div(clock_period_ns)
        .and_then(|ticks| ticks.checked_sub(1))
        .and_then(|ticks| R::try_from(ticks).ok())
        .map(Ticks)
        .ok_or(TicksError::TicksOverflow)
}

/// An opaque value representing a clock frequency
#[derive(Clone, Copy)]
pub struct Frequency(u32);

/// An opaque value representing a clock phase divider
#[derive(Clone, Copy)]
pub struct Divider(u32);

impl Default for Divider {
    fn default() -> Divider {
        Divider(1)
    }
}

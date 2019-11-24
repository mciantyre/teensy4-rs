use imxrt1060_pac as pac;

use core::marker::PhantomData;

pub use pac::GPIO2;
pub use pac::GPIO7;

#[doc(hidden)]
pub trait Init {
    fn init(&mut self);
}

#[doc(hidden)]
pub trait IntoRegisterBlock {
    fn into() -> *const pac::gpio1::RegisterBlock;
}

impl IntoRegisterBlock for pac::GPIO2 {
    fn into() -> *const pac::gpio1::RegisterBlock {
        pac::GPIO2::ptr()
    }
}

impl IntoRegisterBlock for pac::GPIO7 {
    fn into() -> *const pac::gpio1::RegisterBlock {
        pac::GPIO7::ptr()
    }
}

pub struct Ctl<'a, Mux, Pad> {
    mux: &'a Mux,
    pad: &'a Pad,
}

impl<'a> Init
    for Ctl<'a, pac::iomuxc::SW_MUX_CTL_PAD_GPIO_B0_03, pac::iomuxc::SW_PAD_CTL_PAD_GPIO_B0_03>
{
    fn init(&mut self) {
        self.mux.write(|reg| reg.mux_mode().alt5());
        self.pad.write(|reg| reg.dse().dse_7_r0_7());
    }
}

impl<'a, Mux, Pad> From<(&'a Mux, &'a Pad)> for Ctl<'a, Mux, Pad>
where
    Ctl<'a, Mux, Pad>: Init,
{
    fn from((mux, pad): (&'a Mux, &'a Pad)) -> Ctl<'a, Mux, Pad> {
        Ctl { mux, pad }
    }
}

pub struct PinBuilder<Mux, Pad, Reg> {
    _mux: PhantomData<Mux>,
    _pad: PhantomData<Pad>,
    _reg: PhantomData<Reg>,
    offset: u32,
}

impl<Mux, Pad, Reg> PinBuilder<Mux, Pad, Reg> {
    const fn new(offset: u32) -> Self {
        PinBuilder {
            _mux: PhantomData,
            _pad: PhantomData,
            _reg: PhantomData,
            offset,
        }
    }
}

impl<'a, Mux: 'a, Pad: 'a, Reg> PinBuilder<Mux, Pad, Reg>
where
    Ctl<'a, Mux, Pad>: Init,
    Reg: IntoRegisterBlock,
{
    pub fn build<C: Into<Ctl<'a, Mux, Pad>>>(self, ctl: C) -> Pin<Reg> {
        let mut ctl = ctl.into();
        ctl.init();
        Pin::new(self.offset)
    }
}

pub struct Pin<Reg> {
    _reg: PhantomData<Reg>,
    offset: u32,
}

impl<Reg> Pin<Reg>
where
    Reg: IntoRegisterBlock,
{
    fn new(offset: u32) -> Self {
        // TODO this modify could race if there's an interrupt in between the read and
        // writes. Consider an API that could prevent this from happening (accepting a critical section?)
        unsafe {
            (*Reg::into())
                .gdir
                .modify(|state, reg| reg.bits(offset | state.bits()))
        };
        Pin {
            _reg: PhantomData,
            offset,
        }
    }

    /// Toggle the internal state of the pin
    pub fn toggle(&mut self) {
        unsafe { (*Reg::into()).dr_toggle.write(|reg| reg.bits(self.offset)) }
    }

    /// Drive the pin logically low. Note: this may not be electrically low
    pub fn low(&mut self) {
        unsafe { (*Reg::into()).dr_clear.write(|reg| reg.bits(self.offset)) }
    }

    /// Drive the pin to logical high. Note: this may not be electrically high
    pub fn high(&mut self) {
        unsafe { (*Reg::into()).dr_set.write(|reg| reg.bits(self.offset)) }
    }

    /// Returns `true` if logically high, else `false` if logically low
    pub fn state(&self) -> bool {
        unsafe { (*Reg::into()).psr.read().bits() & self.offset > 0 }
    }
}

const fn off(off: u32) -> u32 {
    1 << off
}

pub struct GPIO2Pins {
    pub io3: PinBuilder<
        pac::iomuxc::SW_MUX_CTL_PAD_GPIO_B0_03,
        pac::iomuxc::SW_PAD_CTL_PAD_GPIO_B0_03,
        pac::GPIO2,
    >,
}

impl GPIO2Pins {
    pub(crate) fn new() -> Self {
        GPIO2Pins {
            io3: PinBuilder::new(off(3)),
        }
    }
}

impl Pin<pac::GPIO2> {
    /// Promote the pin to a fast GPIO pin
    ///
    /// The "fast GPIO" bank for GPIO2 is GPIO7.
    pub fn fast_gpio7(self, gpr: &pac::iomuxc_gpr::GPR27) -> Pin<pac::GPIO7> {
        unsafe { gpr.modify(|state, reg| reg.bits(self.offset | state.bits())) };
        Pin::new(self.offset)
    }
}

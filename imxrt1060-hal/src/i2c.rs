//! I2C support

use core::marker::PhantomData;

use crate::ccm;
pub use crate::iomuxc::i2c::module;
use crate::iomuxc::{daisy, i2c};
use imxrt1060_pac as pac;

/// Unclocked I2C modules
///
/// The `Unclocked` struct represents all four unconfigured I2C peripherals.
/// Once clocked, you'll have the ability to build I2C peripherals from the
/// compatible processor pins.
pub struct Unclocked {}
impl Unclocked {
    pub(crate) fn new() -> Self {
        Unclocked {}
    }

    /// Enable clocks to all I2C modules, returning a builder for the four I2C modules.
    pub fn clock(
        self,
        handle: &mut ccm::Handle,
        clock_select: ccm::i2c::ClockSelect,
        divider: ccm::i2c::Divider,
    ) -> (
        Builder<module::_1>,
        Builder<module::_2>,
        Builder<module::_3>,
        Builder<module::_4>,
    ) {
        let (ccm, _) = handle.raw();
        ccm.cscdr2.modify(|_, w| {
            w.lpi2c_clk_podf()
                .variant(divider)
                .lpi2c_clk_sel()
                .variant(clock_select.into())
        });
        ccm.ccgr2.modify(|_, w| unsafe {
            // Safety: each field is 2 bits
            w.cg3().bits(0x3).cg4().bits(0x3).cg5().bits(0x3)
        });
        ccm.ccgr6.modify(|_, w| unsafe {
            // Safety: field is 2 bits
            w.cg12().bits(0x3)
        });
        (
            Builder::new(pac::LPI2C1::ptr()),
            Builder::new(pac::LPI2C2::ptr()),
            Builder::new(pac::LPI2C3::ptr()),
            Builder::new(pac::LPI2C4::ptr()),
        )
    }
}

/// An I2C builder that can build and I2C peripheral
pub struct Builder<M> {
    _module: PhantomData<M>,
    reg: &'static pac::lpi2c1::RegisterBlock,
}

impl<M> Builder<M>
where
    M: module::Module,
{
    fn new(reg: *const pac::lpi2c1::RegisterBlock) -> Self {
        Builder {
            _module: PhantomData,
            // Safety: pointer points to static memory
            reg: unsafe { &*reg },
        }
    }

    /// Builds an I2C peripheral from the SCL and SDA pins. The return
    /// is a configured I2C master running at 100KHz.
    pub fn build<SCL, SDA>(self, mut scl: SCL, mut sda: SDA) -> I2C<M>
    where
        SCL: i2c::Pin<Module = M, Wire = i2c::SCL> + daisy::IntoDaisy,
        SDA: i2c::Pin<Module = M, Wire = i2c::SDA> + daisy::IntoDaisy,
    {
        scl.configure();
        sda.configure();
        let _ = scl.into_daisy();
        let _ = sda.into_daisy();
        I2C::new(self.reg)
    }
}

/// An I2C master
///
/// By default, the I2C master runs at 100KHz, Use `set_clock_speed` to vary
/// the I2C bus speed.
pub struct I2C<M> {
    reg: &'static pac::lpi2c1::RegisterBlock,
    _module: PhantomData<M>,
}

/// I2C Clock speed
#[derive(Clone, Copy)]
pub enum ClockSpeed {
    /// 100 KHz
    KHz100,
    /// 400 KHz
    KHz400,
    /// 1 MHz
    MHz1,
}

/// A handle that will disable I2C master on construction and
/// enable I2C master when dropped. If the I2C master bit
/// was not set, this does nothing.
struct MasterDisabled<'a>(Option<&'a pac::lpi2c1::RegisterBlock>);
impl<'a> MasterDisabled<'a> {
    fn new(reg: &'a pac::lpi2c1::RegisterBlock) -> Self {
        MasterDisabled(if reg.mcr.read().men().bit_is_set() {
            reg.mcr.modify(|_, w| w.men().clear_bit());
            Some(reg)
        } else {
            None
        })
    }
}

impl<'a> Drop for MasterDisabled<'a> {
    fn drop(&mut self) {
        if let Some(reg) = self.0 {
            reg.mcr.modify(|_, w| w.men().set_bit());
        }
    }
}

impl ClockSpeed {
    /// Sets the clock speed parameters
    ///
    /// The function touches I2C registers that should only be touched
    /// while the I2C master is disabled. Enabling the I2C master outside
    /// of the `MasterDisabled` sentinel is a violation of the API.
    fn set(self, _: &MasterDisabled, reg: &pac::lpi2c1::RegisterBlock) {
        match self {
            ClockSpeed::KHz100 => {
                reg.mccr0.write(|w| unsafe {
                    // Safety: 6 bits for all fields
                    w.clkhi()
                        .bits(55)
                        .clklo()
                        .bits(59)
                        .datavd()
                        .bits(25)
                        .sethold()
                        .bits(40)
                });
                reg.mcfgr1.write(|w| w.prescale().prescale_1());
                reg.mcfgr2.write(|w| unsafe {
                    // Safety: 4 bits for filter fields, 12 for busidle
                    w.filtsda().bits(5).filtscl().bits(5).busidle().bits(3900)
                });
            }
            ClockSpeed::KHz400 => {
                reg.mccr0.write(|w| unsafe {
                    // Safety: 6 bits for all fields
                    w.clkhi()
                        .bits(26)
                        .clklo()
                        .bits(28)
                        .datavd()
                        .bits(12)
                        .sethold()
                        .bits(18)
                });
                reg.mcfgr1.write(|w| w.prescale().prescale_0());
                reg.mcfgr2.write(|w| unsafe {
                    // Safety: 4 bits for filter fields, 12 for busidle
                    w.filtsda().bits(2).filtscl().bits(2).busidle().bits(3900)
                });
            }
            ClockSpeed::MHz1 => {
                reg.mccr0.write(|w| unsafe {
                    // Safety: 6 bits for all fields
                    w.clkhi()
                        .bits(9)
                        .clklo()
                        .bits(10)
                        .datavd()
                        .bits(4)
                        .sethold()
                        .bits(7)
                });
                reg.mcfgr1.write(|w| w.prescale().prescale_0());
                reg.mcfgr2.write(|w| unsafe {
                    // Safety: 4 bits for filter fields, 12 for busidle
                    w.filtsda().bits(1).filtscl().bits(1).busidle().bits(3900)
                });
            }
        }
        let mccr0 = reg.mccr0.read().bits();
        reg.mccr1.write(|w| unsafe {
            // Safety: registers are of the same size and fields, just for different purposes
            w.bits(mccr0)
        });
    }
}

impl<M> I2C<M>
where
    M: module::Module,
{
    fn new(reg: &'static pac::lpi2c1::RegisterBlock) -> Self {
        let i2c = I2C {
            reg,
            _module: PhantomData,
        };
        {
            // Disabled master should exist for the entire scope.
            let mdis = MasterDisabled::new(&i2c.reg);
            ClockSpeed::KHz100.set(&mdis, &i2c.reg);
            i2c.reg.mcfgr3.write(|w| unsafe {
                // Safety: pinlow is 12 bits
                w.pinlow().bits(3900)
            });
            i2c.reg.mfcr.write(|w| unsafe {
                // Safety: water marks are 2 bits
                w.rxwater().bits(1).txwater().bits(1)
            });
        }

        // Enable I2C master
        i2c.reg.mcr.modify(|_, w| w.men().set_bit());
        i2c
    }

    /// Set the I2C clock speed to the specified rate
    pub fn set_clock_speed(&mut self, clock_speed: ClockSpeed) {
        let mdis = MasterDisabled::new(&self.reg);
        clock_speed.set(&mdis, &self.reg);
    }
}

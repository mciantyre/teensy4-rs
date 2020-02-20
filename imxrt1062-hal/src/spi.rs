//! SPI support

pub use crate::iomuxc::spi::module;

use crate::ccm;
use crate::iomuxc::{daisy, spi};
use core::marker::PhantomData;
use imxrt1062_pac as pac;
use pac::lpspi1::sr;

/// Unclocked SPI modules
///
/// The `Unclocked` struct represents all four unconfigured SPI peripherals.
/// Once clocked, you'll have the ability to build SPI peripherals from the
/// compatible processor pins.
pub struct Unclocked {}

impl Unclocked {
    pub(crate) fn new() -> Self {
        Unclocked {}
    }

    /// Enable clocks to all SPI modules, returning a builder for the four SPI modules.
    pub fn clock(
        self,
        handle: &mut ccm::Handle,
        clock_select: ccm::spi::ClockSelect,
        divider: ccm::spi::PrescalarSelect,
    ) -> (
        Builder<module::_1>,
        Builder<module::_2>,
        Builder<module::_3>,
        Builder<module::_4>,
    ) {
        let (ccm, _) = handle.raw();
        // First, disable clocks
        ccm.ccgr1.modify(|_, w| unsafe {
            // Safety: each field is 2 bits
            w.cg0()
                .bits(0b00)
                .cg1()
                .bits(0b00)
                .cg2()
                .bits(0b00)
                .cg3()
                .bits(0b00)
        });

        // Select clock, and commit prescalar
        ccm.cbcmr.modify(|_, w| {
            w.lpspi_podf()
                .variant(divider)
                .lpspi_clk_sel()
                .variant(clock_select.into())
        });

        // Enable clocks
        ccm.ccgr1.modify(|_, w| unsafe {
            // Safety: each field is 2 bits
            w.cg0()
                .bits(0b11)
                .cg1()
                .bits(0b11)
                .cg2()
                .bits(0b11)
                .cg3()
                .bits(0b11)
        });

        let source_clock = ccm::Frequency::from(clock_select) / ccm::Divider::from(divider);
        (
            Builder::new(source_clock, pac::LPSPI1::ptr()),
            Builder::new(source_clock, pac::LPSPI2::ptr()),
            Builder::new(source_clock, pac::LPSPI3::ptr()),
            Builder::new(source_clock, pac::LPSPI4::ptr()),
        )
    }
}

/// A SPI builder that can build a SPI peripheral
pub struct Builder<M> {
    _module: PhantomData<M>,
    reg: &'static pac::lpspi1::RegisterBlock,
    /// Frequency of the LPSPI source clock. This
    /// accounts for the divider.
    source_clock: ccm::Frequency,
}

impl<M> Builder<M>
where
    M: module::Module,
{
    fn new(source_clock: ccm::Frequency, reg: *const pac::lpspi1::RegisterBlock) -> Self {
        Builder {
            _module: PhantomData,
            // Safety: pointer points to static memory
            reg: unsafe { &*reg },
            source_clock,
        }
    }

    /// Builds an SPI peripheral from the SDO, SDI, SCK and PCS0 pins. The return
    /// is a configured SPI master running at 8Mhz.
    pub fn build<SDO, SDI, SCK>(self, mut sdo: SDO, mut sdi: SDI, mut sck: SCK) -> SPI<M>
    where
        SDO: spi::Pin<Module = M, Wire = spi::SDO> + daisy::IntoDaisy,
        SDI: spi::Pin<Module = M, Wire = spi::SDI> + daisy::IntoDaisy,
        SCK: spi::Pin<Module = M, Wire = spi::SCK> + daisy::IntoDaisy,
    {
        sdo.configure();
        sdi.configure();
        sck.configure();

        let _ = sdo.into_daisy();
        let _ = sdi.into_daisy();
        let _ = sck.into_daisy();

        SPI::new(self.source_clock, self.reg)
    }
}

/// SPI Clock speed
#[derive(Clone, Copy, Debug)]
pub struct ClockSpeed(pub u32);

impl Default for ClockSpeed {
    fn default() -> Self {
        ClockSpeed(8_000_000)
    }
}

impl ClockSpeed {
    /// Sets the clock speed parameters
    ///
    /// # Safety
    ///
    /// The function touches SPI registers that should only be touched
    /// while the SPI master is disabled.
    unsafe fn set(self, source_clock: ccm::Frequency, reg: &pac::lpspi1::RegisterBlock) {
        log::debug!(
            "SPI baud rate = {:?}, source clock = {:?}",
            self,
            source_clock
        );

        let source_clock_ticks = ccm::Ticks::from(source_clock);
        let mut div = source_clock_ticks.0 / self.0;

        if source_clock_ticks.0 / div > self.0 {
            div += 1;
        }

        // 0 <= div <= 255, and the true coefficient is really div + 2
        let div = div.saturating_sub(2).min(255).max(0) as u8;

        reg.ccr.write(|w|
            // Safety: 0 <= div <= 255, and also of type u8
            w.sckdiv().bits(div)
                // Not sure why we want exactly this delay tbh, but it matches the Arduino SPI lib
                .dbt().bits(div / 2));
    }
}

/// An SPI master
///
/// By default, the SPI master runs at 8Mhz, Use `set_clock_speed` to vary
/// the SPI bus speed.
pub struct SPI<M> {
    reg: &'static pac::lpspi1::RegisterBlock,
    _module: PhantomData<M>,
    /// LPSPI effective input clock frequency
    source_clock: ccm::Frequency,
}

/// Indicates an error when computing the parameters that control
/// the clock speed.
#[derive(Debug)]
pub struct ClockSpeedError;

/// Indicates an error when computing the parameters that control
/// the mode.
#[derive(Debug)]
pub struct ModeError;

/// Indicates an error when computing the parameters that control
/// the pin low timeout
#[derive(Debug)]
pub struct PinLowTimeoutError;

/// Indicates an error when computing the parameters that control
/// the bus idle timeout
#[derive(Debug)]
pub struct BusIdleTimeoutError;

const RETRIES: usize = 100_000;

impl<M> SPI<M>
where
    M: module::Module,
{
    fn new(source_clock: ccm::Frequency, reg: &'static pac::lpspi1::RegisterBlock) -> Self {
        let mut spi = SPI {
            reg,
            _module: PhantomData,
            source_clock,
        };
        spi.reg.cr.write_with_zero(|w| w.rst().set_bit());
        // Enables SPI master
        spi.set_clock_speed(ClockSpeed::default()).unwrap();
        spi.reg
            .cfgr1
            .write(|w| w.master().master_1().sample().sample_1());
        spi.set_mode(embedded_hal::spi::MODE_0).unwrap();
        spi.reg.fcr.write(|w| unsafe {
            // Safety: fields are four bits
            w.rxwater().bits(0xf).txwater().bits(0xf)
        });
        spi
    }

    fn with_master_disabled<F: FnMut() -> R, R>(&self, mut act: F) -> R {
        self.reg.cr.reset();
        let res = act();
        self.reg.cr.write_with_zero(|w| w.men().set_bit());
        res
    }

    pub fn set_mode(&mut self, mode: embedded_hal::spi::Mode) -> Result<(), ModeError> {
        self.with_master_disabled(|| unsafe {
            self.reg.tcr.write(|w| {
                w.framesz()
                    .bits(0xf)
                    .cpol()
                    .bit(mode.polarity == embedded_hal::spi::Polarity::IdleHigh)
                    .cpha()
                    .bit(mode.phase == embedded_hal::spi::Phase::CaptureOnSecondTransition)
            });
            Ok(())
        })
    }

    /// Set the SPI master clock speed
    pub fn set_clock_speed(&mut self, clock_speed: ClockSpeed) -> Result<(), ClockSpeedError> {
        self.with_master_disabled(|| unsafe {
            // Safety: master is disabled
            clock_speed.set(self.source_clock, self.reg);
            Ok(())
        })
    }

    #[inline(always)]
    fn wait<F>(&mut self, mut on: F) -> Result<(), Error>
    where
        F: FnMut(sr::R) -> bool,
    {
        for _ in 0..RETRIES {
            if on(self.check_errors()?) {
                return Ok(());
            }
        }
        Err(Error::WaitTimeout)
    }

    /// Clears all master status flags that are required to be
    /// low before acting as an SPI master.
    ///
    /// All flags are W1C.
    fn clear_status(&mut self) {
        self.reg.sr.write(|w| {
            w.wcf()
                .wcf_1()
                .fcf()
                .fcf_1()
                .tcf()
                .tcf_1()
                .tef()
                .tef_1()
                .ref_()
                .ref_1()
                .dmf()
                .dmf_1()
        });
    }

    // TODO: for now I believe this is required to be public for the cases where an user wishes
    // to clear the FIFO.  It would be a bit cleaner if we had SPI transaction methods
    pub fn clear_fifo(&mut self) {
        self.reg.cr.modify(|_, w| w.rrf().set_bit().rtf().set_bit());
    }

    /// Check master status flags for erroneous conditions
    #[inline(always)]
    fn check_errors(&mut self) -> Result<sr::R, Error> {
        let status = self.reg.sr.read();
        if status.tef().bit_is_set() {
            Err(Error::Transmit)
        } else if status.ref_().bit_is_set() {
            Err(Error::Receive)
        } else if status.dmf().bit_is_set() {
            Err(Error::DataMismatch) // TODO: is this an error?
        } else {
            Ok(status)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// A generic transmit error
    Transmit,
    /// A generic receive error
    Receive,
    /// Data mismatch error
    DataMismatch,
    /// Requesting too much data in a receive; upper limit is `u8::MAX`
    RequestTooMuchData,
    /// Busy-wait on an internal flag was too long
    WaitTimeout,
}

impl<M> embedded_hal::spi::FullDuplex<u8> for SPI<M>
where
    M: module::Module,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let sr = self.check_errors()?;
        if sr.mbf().bit_is_set() {
            return Err(nb::Error::WouldBlock);
        }

        if self.reg.rsr.read().rxempty().bit_is_clear() {
            let word = self.reg.rdr.read().data().bits();
            Ok(word as u8)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    /// Sends a word to the slave
    fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        let sr = self.check_errors()?;
        self.clear_status();
        if sr.mbf().bit_is_set() || sr.tdf().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        }

        self.reg
            .tdr
            .write(|w| unsafe { w.data().bits(word as u32) });
        self.wait(|msr| msr.tdf().bit_is_set())?;
        Ok(())
    }
}

impl<M> embedded_hal::blocking::spi::write::Default<u8> for SPI<M> where M: module::Module {}

impl<M> embedded_hal::blocking::spi::transfer::Default<u8> for SPI<M> where M: module::Module {}

impl<M> embedded_hal::blocking::spi::write_iter::Default<u8> for SPI<M> where M: module::Module {}

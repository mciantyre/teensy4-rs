//! I2C support

pub use crate::iomuxc::i2c::module;

use crate::ccm;
use crate::iomuxc::{daisy, i2c};
use core::convert::TryFrom;
use core::marker::PhantomData;
use embedded_hal::blocking;
use imxrt1060_pac as pac;
use pac::lpi2c1::mtdr::CMD_AW;

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
        divider: ccm::i2c::PrescalarSelect,
    ) -> (
        Builder<module::_1>,
        Builder<module::_2>,
        Builder<module::_3>,
        Builder<module::_4>,
    ) {
        let (ccm, _) = handle.raw();
        // First, disable clocks
        ccm.ccgr2.modify(|_, w| unsafe {
            // Safety: each field is 2 bits
            w.cg3().bits(0x0).cg4().bits(0x0).cg5().bits(0x0)
        });
        ccm.ccgr6.modify(|_, w| unsafe {
            // Safety: field is 2 bits
            w.cg12().bits(0x0)
        });
        // Select clock, and commit prescalar
        ccm.cscdr2.modify(|_, w| {
            w.lpi2c_clk_podf()
                .variant(divider)
                .lpi2c_clk_sel()
                .variant(clock_select.into())
        });
        // Enable clocks
        ccm.ccgr2.modify(|_, w| unsafe {
            // Safety: each field is 2 bits
            w.cg3().bits(0x3).cg4().bits(0x3).cg5().bits(0x3)
        });
        ccm.ccgr6.modify(|_, w| unsafe {
            // Safety: field is 2 bits
            w.cg12().bits(0x3)
        });
        let source_clock = ccm::Frequency::from(clock_select) / ccm::Divider::from(divider);
        (
            Builder::new(source_clock, pac::LPI2C1::ptr()),
            Builder::new(source_clock, pac::LPI2C2::ptr()),
            Builder::new(source_clock, pac::LPI2C3::ptr()),
            Builder::new(source_clock, pac::LPI2C4::ptr()),
        )
    }
}

/// An I2C builder that can build and I2C peripheral
pub struct Builder<M> {
    _module: PhantomData<M>,
    reg: &'static pac::lpi2c1::RegisterBlock,
    /// Frequency of the LPI2C source clock. This
    /// accounts for the divider.
    source_clock: ccm::Frequency,
}

impl<M> Builder<M>
where
    M: module::Module,
{
    fn new(source_clock: ccm::Frequency, reg: *const pac::lpi2c1::RegisterBlock) -> Self {
        Builder {
            _module: PhantomData,
            // Safety: pointer points to static memory
            reg: unsafe { &*reg },
            source_clock,
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
        I2C::new(self.source_clock, self.reg)
    }
}

/// I2C Clock speed
#[derive(Clone, Copy, Debug)]
pub enum ClockSpeed {
    /// 100 KHz
    KHz100,
    /// 400 KHz
    KHz400,
    /// 1 MHz
    MHz1,
}

impl Default for ClockSpeed {
    fn default() -> Self {
        ClockSpeed::KHz100
    }
}

impl ClockSpeed {
    /// Sets the clock speed parameters
    ///
    /// # Safety
    ///
    /// The function touches I2C registers that should only be touched
    /// while the I2C master is disabled.
    unsafe fn set(self, source_clock: ccm::Frequency, reg: &pac::lpi2c1::RegisterBlock) {
        // Baud rate = (source_clock/2^prescale)/(CLKLO+1+CLKHI+1 + FLOOR((2+FILTSCL)/2^prescale)
        // Assume CLKLO = 2*CLKHI, SETHOLD = CLKHI, DATAVD = CLKHI/2, FILTSCL = FILTSDA = 0,
        // and that risetime is negligible (less than 1 cycle).
        use core::cmp;
        use pac::lpi2c1::mcfgr1::PRESCALE_A;

        log::debug!(
            "I2C baud rate = {:?}, source clock = {:?}",
            self,
            source_clock
        );

        const PRESCALARS: [PRESCALE_A; 8] = [
            PRESCALE_A::PRESCALE_0,
            PRESCALE_A::PRESCALE_1,
            PRESCALE_A::PRESCALE_2,
            PRESCALE_A::PRESCALE_3,
            PRESCALE_A::PRESCALE_4,
            PRESCALE_A::PRESCALE_5,
            PRESCALE_A::PRESCALE_6,
            PRESCALE_A::PRESCALE_7,
        ];

        struct ByError {
            prescalar: PRESCALE_A,
            clkhi: u8,
            error: u32,
            computed_rate: u32,
        }

        let baud_rate = match self {
            Self::KHz100 => ccm::Ticks(100_000),
            Self::KHz400 => ccm::Ticks(400_000),
            Self::MHz1 => ccm::Ticks(1_000_000),
        };

        // prescale = 1, 2, 4, 8, ... 128
        // divider = 2 ^ prescale
        let dividers = PRESCALARS.into_iter().copied().map(ccm::Divider::from);
        let clkhis = (1u32..32u32).map(ccm::Ticks);
        // possibilities = every divider with every clkhi (8 * 30 == 240 possibilities)
        let possibilities =
            dividers.flat_map(|divider| core::iter::repeat(divider).zip(clkhis.clone()));
        let errors = possibilities.map(|(divider, clkhi)| {
            let computed_rate = if ccm::Ticks(1) == clkhi {
                // See below for justification on magic numbers.
                // In the 1 == clkhi case, the + 3 is the minimum allowable CLKLO value
                // + 1 is CLKHI itself
                ccm::Ticks::from(source_clock / divider)
                    / (ccm::Ticks(1 + 3 + 2) + ccm::Ticks(2) / divider)
            } else {
                // CLKLO = 2 * CLKHI, allows us to do 3 * CLKHI
                // + 2 accounts for the CLKLOW + 1 and CLKHI + 1
                // + 2 accounts for the FLOOR((2 + FILTSCL)) factor
                ccm::Ticks::from(source_clock / divider)
                    / (ccm::Ticks(3 * clkhi.0 + 2) + ccm::Ticks(2) / divider)
            };
            let error = cmp::max(computed_rate, baud_rate) - cmp::min(computed_rate, baud_rate);
            ByError {
                prescalar: PRESCALE_A::from(divider),
                clkhi: clkhi.0 as u8, /* (1..32) in u8 range */
                error: error.0,
                computed_rate: computed_rate.0,
            }
        });

        let ByError {
            prescalar,
            clkhi,
            error,
            computed_rate,
        } = errors.min_by(|lhs, rhs| lhs.error.cmp(&rhs.error)).unwrap();

        let (clklo, sethold, datavd) = if clkhi < 2 {
            (3, 2, 1)
        } else {
            (clkhi * 2, clkhi, clkhi / 2)
        };

        log::debug!(
            "COMPUTED_RATE = {}, ERROR = {}, PRESCALAR = {:?}, CLKHI = {}, CLKLO = {}, SETHOLD = {}, DAVAVD = {}",
            computed_rate,
            error,
            prescalar,
            clkhi,
            clklo,
            sethold,
            datavd
        );
        reg.mccr0.modify(|_, w| {
            // Safety: fields are 6 bits
            w.clkhi()
                .bits(clkhi)
                .clklo()
                .bits(clklo)
                .sethold()
                .bits(sethold)
                .datavd()
                .bits(datavd)
        });
        reg.mcfgr1.modify(|_, w| w.prescale().variant(prescalar));
    }
}

/// An I2C master
///
/// By default, the I2C master runs at 100KHz, Use `set_clock_speed` to vary
/// the I2C bus speed.
pub struct I2C<M> {
    reg: &'static pac::lpi2c1::RegisterBlock,
    _module: PhantomData<M>,
    /// LPI2C effective input clock frequency
    source_clock: ccm::Frequency,
}

/// Indicates an error when computing the parameters that control
/// the clock speed.
#[derive(Debug)]
pub struct ClockSpeedError;
/// Indicates an error when computing the parameters that control
/// the pin low timeout
#[derive(Debug)]
pub struct PinLowTimeoutError;
/// Indicates an error when computing the parameters that control
/// the bus idle timeout
#[derive(Debug)]
pub struct BusIdleTimeoutError;

impl<M> I2C<M>
where
    M: module::Module,
{
    fn new(source_clock: ccm::Frequency, reg: &'static pac::lpi2c1::RegisterBlock) -> Self {
        let mut i2c = I2C {
            reg,
            _module: PhantomData,
            source_clock,
        };
        // Enables I2C master
        i2c.set_clock_speed(ClockSpeed::KHz100).unwrap();
        i2c
    }

    fn with_master_disabled<F: FnMut() -> R, R>(&self, mut act: F) -> R {
        self.reg.mcr.modify(|_, w| w.men().clear_bit());
        let res = act();
        self.reg.mcr.modify(|_, w| w.men().set_bit());
        res
    }

    /// Set the I2C master clock speed
    pub fn set_clock_speed(&mut self, clock_speed: ClockSpeed) -> Result<(), ClockSpeedError> {
        self.with_master_disabled(|| unsafe {
            // Safety: master is disabled
            clock_speed.set(self.source_clock, self.reg);
            Ok(())
        })
    }

    /// Set the pin low timeout
    ///
    /// If SCL or, either SCL or SDA, is low for longer than the specified duration, then the
    /// I2C hardware indicates an error. If the timeout is `0`, then the detection is disabled.
    ///
    /// If the number of cycles required to represent the duration is too large, returns a
    /// `PinLowTimeoutError`. Try using a smaller duration.
    pub fn set_pin_low_timeout(
        &mut self,
        timeout: core::time::Duration,
    ) -> Result<(), PinLowTimeoutError> {
        let divider = self.reg.mcfgr1.read().prescale().variant().into();
        let pin_low_ticks: ccm::Ticks<u16> = ccm::ticks(timeout, self.source_clock, divider)
            .map(|ticks: ccm::Ticks<u16>| ccm::Ticks(ticks.0 / 256))
            .into_iter()
            .next()
            .filter(|ticks| *ticks <= ccm::Ticks(0x0FFFu16))
            .ok_or(PinLowTimeoutError)?;
        log::debug!("PINLOW = 0x{:X}", pin_low_ticks.0);
        self.with_master_disabled(|| {
            self.reg.mcfgr3.modify(|_, w| unsafe {
                // Safety: pinlow is 12 bits
                w.pinlow().bits(pin_low_ticks.0)
            });
            Ok(())
        })
    }

    /// Set the bus idle timeout
    ///
    /// If both SCL and SDA are high for longer than the timeout, then the I2C bus is assumed to be
    /// idle and the master can generate a START condition. If the timeout is `0`, then the idle is
    /// disabled.
    ///
    /// If the number of cycles required to represent the duration is too large, returns a
    /// `BusIdleTimeoutError`. Try using a smaller timeout.
    pub fn set_bus_idle_timeout(
        &mut self,
        timeout: core::time::Duration,
    ) -> Result<(), BusIdleTimeoutError> {
        let divider = self.reg.mcfgr1.read().prescale().variant().into();
        let bus_idle_ticks: ccm::Ticks<u16> = ccm::ticks(timeout, self.source_clock, divider)
            .into_iter()
            .next()
            .filter(|ticks| *ticks <= ccm::Ticks(0xFFFu16))
            .ok_or(BusIdleTimeoutError)?;
        log::debug!("BUSIDLE = 0x{:X}", bus_idle_ticks.0);
        self.with_master_disabled(|| {
            self.reg.mcfgr2.modify(|_, w| unsafe {
                // Safety: busidle is 12 bits
                w.busidle().bits(bus_idle_ticks.0)
            });
            Ok(())
        })
    }

    #[inline(always)]
    fn wait_idle(&mut self) {
        log::trace!("Waiting for I2C IDLE...");
        loop {
            let status = self.reg.msr.read();
            // Bus is idle
            if status.bbf().bit_is_clear()
            // Master is busy; we already have the bus
            || status.mbf().bit_is_set()
            {
                log::trace!("I2C bus IDLE");
                break;
            }
        }
    }

    #[inline(always)]
    fn wait_stop(&mut self) {
        log::trace!("Waiting for I2C STOP...");
        loop {
            let status = self.reg.msr.read();
            if status.sdf().bit_is_set() {
                log::trace!("I2C STOP asserted");
                break;
            }
        }
    }

    /// Clears all master status flags that are required to be
    /// low before acting as an I2C master.
    ///
    /// All flags are W1C.
    #[inline(always)]
    fn clear_status(&mut self) {
        self.reg.msr.write(|w| {
            w.epf()
                .epf_1()
                .sdf()
                .sdf_1()
                .ndf()
                .ndf_1()
                .alf()
                .alf_1()
                .fef()
                .fef_1()
                .pltf()
                .pltf_1()
                .dmf()
                .dmf_1()
        });
    }

    /// Check master status flags for erroneous conditions
    #[inline(always)]
    fn check_errors(&mut self) -> Result<(), Error> {
        let status = self.reg.msr.read();
        if status.pltf().bit_is_set() {
            Err(Error::PinLowTimeout)
        } else if status.alf().bit_is_set() {
            Err(Error::LostBusArbitration)
        } else if status.ndf().bit_is_set() {
            Err(Error::UnexpectedNACK)
        } else if status.fef().bit_is_set() {
            Err(Error::FIFO)
        } else {
            Ok(())
        }
    }

    // Enqueues all the commands in the MTDR FIFO
    fn enqueue_mtdr<C>(&mut self, mut commands: C) -> Result<(), Error>
    where
        C: Iterator<Item = Command>,
    {
        const FIFO_SIZE: u8 = 4;

        commands.try_for_each(|command| loop {
            self.check_errors()?;
            let fifo_used = self.reg.mfsr.read().txcount().bits();
            if fifo_used < FIFO_SIZE {
                self.reg.mtdr.write(|w| {
                    w.cmd().variant(command.into());
                    match command {
                        Command::Start(data)   |
                        Command::Data(data)    |
                        Command::Receive(data) => unsafe {
                            // Safety: data field is 8 bits
                            w.data().bits(data)
                        },
                        Command::Stop => w,
                    }
                });
                return Ok(());
            }
        })
    }

    /// Retrieves data from the MRDR FIFO, placing each byte in the buffer
    fn retrieve_mrdr<'a, C>(&mut self, mut buffer: C) -> Result<(), Error>
    where
        C: Iterator<Item = &'a mut u8>,
    {
        buffer.try_for_each(|slot| loop {
            self.check_errors()?;
            let mrdr = self.reg.mrdr.read();
            if mrdr.rxempty().bit_is_clear() {
                *slot = mrdr.data().bits();
                return Ok(());
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Master has lost arbitration
    LostBusArbitration,
    /// SCL and / or SDA went low for too long, despite our control
    PinLowTimeout,
    /// Detected a NACK when sending an address or data
    UnexpectedNACK,
    /// Sending or receiving data without a START
    FIFO,
    /// Requesting too much data in a receive; upper limit is `u8::MAX`
    RequestTooMuchData,
}

const READ: u8 = 1;
const WRITE: u8 = 0;

#[derive(Clone, Copy)]
enum Command {
    Start(u8),
    Data(u8),
    Receive(u8),
    Stop,
}

impl Command {
    fn start(addr: u8, direction: u8) -> Self {
        Command::Start(addr << 1 | direction)
    }
    fn receive(len: u8) -> Self {
        // Receive commands are for the value + 1, so we subtract
        // 1 to account for the off-by-one
        Command::Receive(len.saturating_sub(1))
    }
}

impl From<Command> for CMD_AW {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Start(..) => CMD_AW::CMD_4,
            Command::Data(..) => CMD_AW::CMD_0,
            Command::Receive(_) => CMD_AW::CMD_1,
            Command::Stop => CMD_AW::CMD_2,
        }
    }
}

impl<M> blocking::i2c::Write for I2C<M>
where
    M: module::Module,
{
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        <Self as blocking::i2c::WriteIter>::write(self, addr, bytes.iter().cloned())
    }
}

impl<M> blocking::i2c::WriteIter for I2C<M>
where
    M: module::Module,
{
    type Error = Error;

    fn write<I>(&mut self, addr: u8, bytes: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        self.wait_idle();
        self.clear_status();

        let transmission = core::iter::once(Command::start(addr, WRITE))
            .chain(bytes.into_iter().map(Command::Data))
            .chain(core::iter::once(Command::Stop));

        self.enqueue_mtdr(transmission)?;
        self.wait_stop();
        Ok(())
    }
}

impl<M> blocking::i2c::WriteRead for I2C<M>
where
    M: module::Module,
{
    type Error = Error;
    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        use blocking::i2c::WriteIterRead;
        self.write_iter_read(address, bytes.iter().cloned(), buffer)
    }
}

impl<M> blocking::i2c::WriteIterRead for I2C<M>
where
    M: module::Module,
{
    type Error = Error;

    fn write_iter_read<B>(
        &mut self,
        address: u8,
        bytes: B,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        let receive_len = u8::try_from(buffer.len()).map_err(|_| Error::RequestTooMuchData)?;

        self.wait_idle();
        self.clear_status();

        self.enqueue_mtdr(
            core::iter::once(Command::start(address, WRITE))
                .chain(bytes.into_iter().map(Command::Data)),
        )?;

        if receive_len > 0 {
            let recv_cmds = [Command::start(address, READ), Command::receive(receive_len)];
            self.enqueue_mtdr(recv_cmds.iter().cloned())?;
            self.retrieve_mrdr(buffer.iter_mut())?;
        }
        self.enqueue_mtdr(core::iter::once(Command::Stop))?;
        self.wait_stop();
        Ok(())
    }
}

impl<M> blocking::i2c::Read for I2C<M>
where
    M: module::Module,
{
    type Error = Error;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let receive_len = u8::try_from(buffer.len()).map_err(|_| Error::RequestTooMuchData)?;
        if receive_len == 0 {
            return Ok(());
        }

        self.wait_idle();
        self.clear_status();
        let recv_cmds = [Command::start(address, READ), Command::receive(receive_len)];
        self.enqueue_mtdr(recv_cmds.iter().cloned())?;
        self.retrieve_mrdr(buffer.iter_mut())?;
        self.enqueue_mtdr(core::iter::once(Command::Stop))?;
        self.wait_stop();
        Ok(())
    }
}

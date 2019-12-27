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

impl ClockSpeed {
    /// Sets the clock speed parameters
    ///
    /// # Safety
    ///
    /// The function touches I2C registers that should only be touched
    /// while the I2C master is disabled. Enabling the I2C master outside
    /// of the `MasterDisabled` sentinel is a violation of the API.
    unsafe fn set(self, reg: &pac::lpi2c1::RegisterBlock) {
        match self {
            ClockSpeed::KHz100 => {
                reg.mccr0.write(|w| {
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
                reg.mcfgr2.write(|w| {
                    // Safety: 4 bits for filter fields, 12 for busidle
                    w.filtsda().bits(5).filtscl().bits(5).busidle().bits(3900)
                });
            }
            ClockSpeed::KHz400 => {
                reg.mccr0.write(|w| {
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
                reg.mcfgr2.write(|w| {
                    // Safety: 4 bits for filter fields, 12 for busidle
                    w.filtsda().bits(2).filtscl().bits(2).busidle().bits(3900)
                });
            }
            ClockSpeed::MHz1 => {
                reg.mccr0.write(|w| {
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
                reg.mcfgr2.write(|w| {
                    // Safety: 4 bits for filter fields, 12 for busidle
                    w.filtsda().bits(1).filtscl().bits(1).busidle().bits(3900)
                });
            }
        }
        let mccr0 = reg.mccr0.read().bits();
        reg.mccr1.write(|w| {
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

        i2c.reg.mcr.modify(|_, w| w.men().clear_bit());
        // Safety: modified while master is diabled
        unsafe {
            ClockSpeed::KHz100.set(&i2c.reg);
        }
        i2c.reg.mcfgr3.write(|w| unsafe {
            // Safety: pinlow is 12 bits
            w.pinlow().bits(3900)
        });
        i2c.reg.mfcr.write(|w| unsafe {
            // Safety: water marks are 2 bits
            w.rxwater().bits(1).txwater().bits(0)
        });

        // Enable I2C master
        i2c.reg.mcr.modify(|_, w| w.men().set_bit());
        i2c
    }

    /// Set the I2C clock speed to the specified rate
    pub fn set_clock_speed(&mut self, clock_speed: ClockSpeed) {
        self.reg.mcr.modify(|_, w| w.men().clear_bit());
        // Safety: called while master is disabled
        unsafe {
            clock_speed.set(&self.reg);
        }
        self.reg.mcr.modify(|_, w| w.men().set_bit());
    }

    #[inline(always)]
    fn wait_idle(&mut self) {
        loop {
            let status = self.reg.msr.read();
            if status.bbf().bit_is_clear() // Bus is idle
            || status.mbf().bit_is_set()
            {
                // Master is busy; we already have the bus
                break;
            }
        }
    }

    #[inline(always)]
    fn wait_stop(&mut self) {
        loop {
            let status = self.reg.msr.read();
            if status.sdf().bit_is_set() {
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

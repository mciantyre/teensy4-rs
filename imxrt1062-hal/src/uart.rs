//! Low Power Universal Asynchronous Receiver/Transmit

use crate::ccm;
pub use crate::iomuxc::uart::{self, module};
use crate::pac;
use core::marker::PhantomData;

pub struct Uninit<M> {
    effective_clock: ccm::Frequency,
    _module: PhantomData<M>,
    reg: &'static pac::lpuart1::RegisterBlock,
}

impl<M> Uninit<M> {
    fn new(effective_clock: ccm::Frequency, reg: *const pac::lpuart1::RegisterBlock) -> Self {
        Uninit {
            effective_clock,
            _module: PhantomData,
            // Safety: register is static
            reg: unsafe { &*reg },
        }
    }
}

pub struct UARTs {
    pub uart1: Uninit<module::_1>,
    pub uart3: Uninit<module::_3>,
    pub uart6: Uninit<module::_6>,
}

pub struct Unclocked {}
impl Unclocked {
    pub(crate) fn new() -> Self {
        Unclocked {}
    }

    pub fn clock(
        self,
        ccm: &mut ccm::Handle,
        clock_select: ccm::uart::ClockSelect,
        prescalar: ccm::uart::PrescalarSelect,
    ) -> UARTs {
        let (ccm, _) = ccm.raw();

        // Disable clocks before modifying selection
        ccm.ccgr5.modify(|_, w| unsafe { w.cg12().bits(0b00) }); // UART1
        ccm.ccgr0.modify(|_, w| unsafe { w.cg6().bits(0b00) }); // UART3
        ccm.ccgr3.modify(|_, w| unsafe { w.cg3().bits(0b00) }); // UART6

        // Select clocks & prescalar
        ccm.cscdr1.write(|w| {
            w.uart_clk_sel()
                .variant(clock_select.into())
                .uart_clk_podf()
                .variant(prescalar.into())
        });

        // Enable clocks
        ccm.ccgr5.modify(|_, w| unsafe { w.cg12().bits(0b11) }); // UART1
        ccm.ccgr0.modify(|_, w| unsafe { w.cg6().bits(0b11) }); // UART3
        ccm.ccgr3.modify(|_, w| unsafe { w.cg3().bits(0b11) }); // UART6

        let effective_clock = ccm::Frequency::from(clock_select) / ccm::Divider::from(prescalar);
        UARTs {
            uart1: Uninit::new(effective_clock, pac::LPUART1::ptr()),
            uart3: Uninit::new(effective_clock, pac::LPUART3::ptr()),
            uart6: Uninit::new(effective_clock, pac::LPUART6::ptr()),
        }
    }
}

impl<M> Uninit<M>
where
    M: module::Module,
{
    /// Initializes a UART on the `tx` and `rx` pins. Specify the initial
    /// baud rate of the bus with `baud`. Returns the configured UART
    /// peripheral, or an error that indicates we could not configure the
    /// baud rate.
    pub fn init<TX, RX>(
        self,
        mut tx: TX,
        mut rx: RX,
        baud: u32,
    ) -> Result<UART<M>, ccm::uart::TimingsError>
    where
        TX: uart::Pin<Direction = uart::TX, Module = M>,
        RX: uart::Pin<Direction = uart::RX, Module = M>,
    {
        tx.configure();
        rx.configure();
        UART::start(self.reg, self.effective_clock, baud)
    }
}

/// An initialized UART peripheral
pub struct UART<M> {
    reg: &'static pac::lpuart1::RegisterBlock,
    effective_clock: ccm::Frequency,
    _module: PhantomData<M>,
}

impl<M> UART<M>
where
    M: module::Module,
{
    fn start(
        reg: &'static pac::lpuart1::RegisterBlock,
        effective_clock: ccm::Frequency,
        baud: u32,
    ) -> Result<Self, ccm::uart::TimingsError> {
        let mut uart = UART {
            reg,
            effective_clock,
            _module: PhantomData,
        };
        uart.set_baud(baud)?;
        uart.reg.ctrl.modify(|_, w| w.te().set_bit().re().set_bit());
        Ok(uart)
    }

    /// Controls the TX FIFO.
    ///
    /// If size is `Some(n)`, where `n > 0`, the method will enable the TX
    /// FIFO with a size `n`. The method returns size of the FIFO that was
    /// set, which is based on the hardware. On an iMXRT1062, the max size
    /// is 4.
    ///
    /// If size is `None`, the method disables the TX FIFO. The return is 0.
    ///
    /// The method temporarily disables the UART bus, flushing any data in
    /// the TX FIFO.
    pub fn set_tx_fifo(&mut self, size: Option<core::num::NonZeroU8>) -> u8 {
        self.while_disabled(|this| {
            if let Some(requested_size) = size {
                // Maximum TX FIFO size supported by this device
                let max_size = this.reg.param.read().txfifo().bits() << 1;
                let tx_fifo_size = max_size.min(requested_size.get());
                this.reg.water.modify(|_, w| unsafe {
                    // Safety: max size is one less than PARAM[TXFIFO].
                    // Assume an iMXRT1062. PARAM[TXFIFO] = 4, so
                    // WATER[TXWATER] = 3. 3 == 0b11, which fits into
                    // the two bit range of the field. We're assuming
                    // that this scales for chips that might have a larger
                    // PARAM[TXFIFO] size.
                    w.txwater().bits(tx_fifo_size - 1)
                });
                this.reg.fifo.modify(|_, w| w.txfe().set_bit());
                tx_fifo_size
            } else {
                this.reg.fifo.modify(|_, w| w.txfe().clear_bit());
                0
            }
        })
    }

    pub fn set_rx_fifo(&mut self, enable: bool) {
        self.while_disabled(|this| {
            this.reg.fifo.modify(|_, w| w.rxfe().bit(enable));
        })
    }

    fn while_disabled<F: FnMut(&mut Self) -> R, R>(&mut self, mut act: F) -> R {
        let mut was_enabled = false;
        self.reg
            .fifo
            .modify(|_, w| w.txflush().set_bit().rxflush().set_bit());
        self.reg.ctrl.modify(|r, w| {
            was_enabled = r.te().bit_is_set() && r.re().bit_is_set();
            w.te().clear_bit().re().clear_bit()
        });
        let res = act(self);
        self.reg
            .ctrl
            .modify(|_, w| w.te().bit(was_enabled).re().bit(was_enabled));
        res
    }

    /// Set the baud rate for the UART bus. Returns a `TimingsError` if there was
    /// an error computing the values that describe the baud rate.
    pub fn set_baud(&mut self, baud: u32) -> Result<(), ccm::uart::TimingsError> {
        let timings = ccm::uart::timings(self.effective_clock, baud)?;
        self.while_disabled(|this| {
            this.reg.baud.modify(|_, w| unsafe {
                // Safety: correctness of the values depends on the return of
                // ccm::uart::timings
                w.osr()
                    .bits(timings.osr)
                    .sbr()
                    .bits(timings.sbr)
                    .bothedge()
                    .bit(timings.both_edge)
            });
            this.reg.ctrl.modify(|_, w| w.te().set_bit().re().set_bit());
        });
        Ok(())
    }

    fn clear_status(&mut self) {
        self.reg.stat.modify(|_, w| {
            w.idle()
                .set_bit()
                .or()
                .set_bit()
                .nf()
                .set_bit()
                .fe()
                .set_bit()
                .pf()
                .set_bit()
        });
    }
}

use embedded_hal::serial;

impl<M> serial::Write<u8> for UART<M>
where
    M: module::Module,
{
    type Error = core::convert::Infallible;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.flush()?;
        self.reg.data.write(|w| unsafe { w.bits(word as u32) });
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if self.reg.stat.read().tdre().is_tdre_0() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}

bitflags::bitflags! {
    /// Errors that may occur when reading data
    pub struct ReadErrorFlags : u8 {
        /// Data was received with noise
        const NOISY = 1 << 7;
        /// Parity error when receiving data
        const PARITY = 1 << 6;
        /// Framing error when receiving data
        const FRAME_ERROR = 1 << 5;
        /// Overrun occured, and we lost data in the shift register
        const OVERRUN = 1 << 4;
    }
}

/// Type that describes a read error
pub struct ReadError {
    /// Decribes the reason for the error
    pub flags: ReadErrorFlags,
    /// The raw value read, if you'd like to consider it
    pub raw: u8,
}

impl<M> serial::Read<u8> for UART<M>
where
    M: module::Module,
{
    type Error = ReadError;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let data = self.reg.data.read();
        if data.rxempt().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            let mut flags = ReadErrorFlags::empty();
            flags.set(ReadErrorFlags::OVERRUN, self.reg.stat.read().or().bit());
            flags.set(ReadErrorFlags::PARITY, data.paritye().bit());
            flags.set(ReadErrorFlags::FRAME_ERROR, data.fretsc().bit());
            flags.set(ReadErrorFlags::NOISY, data.noisy().bit());

            let raw = (data.bits() & 0xFF) as u8;
            self.clear_status();

            if flags.is_empty() {
                Ok(raw)
            } else {
                Err(nb::Error::Other(ReadError { flags, raw }))
            }
        }
    }
}

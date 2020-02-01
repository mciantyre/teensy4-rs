//! Low Power Universal Asynchronous Receiver/Transmit (LPUART)
//!
//! The UART module provides a serial peripheral that implements
//! the `embedded_hal::serial` traits. The peripheral is sufficient
//! for implementing basic serial communications.

use crate::ccm;
pub use crate::iomuxc::uart::{self, module};
use crate::pac;
use core::marker::PhantomData;

/// An uninitialized UART peripheral
///
/// Call `init()` to initialize the peripheral
pub struct Uninit<M: module::Module> {
    effective_clock: ccm::Frequency,
    _module: PhantomData<M>,
    reg: M::Reg,
}

impl<M: module::Module> Uninit<M> {
    fn new(effective_clock: ccm::Frequency, reg: M::Reg) -> Self {
        Uninit {
            effective_clock,
            _module: PhantomData,
            reg,
        }
    }
}

/// All available UARTs
///
/// All UARTs are uninitialized. Call `init()` to take and initialize the
/// peripheral.
pub struct UARTs {
    pub uart1: Uninit<module::_1>,
    pub uart2: Uninit<module::_2>,
    pub uart3: Uninit<module::_3>,
    pub uart4: Uninit<module::_4>,
    pub uart5: Uninit<module::_5>,
    pub uart6: Uninit<module::_6>,
    pub uart7: Uninit<module::_7>,
    pub uart8: Uninit<module::_8>,
}

/// Unclocked UART peripherals
///
/// The `Unclocked` UART represents all UART peripherals
/// that do not have an activated clock. In order to obtain
/// any UART peripheral, the `Unclocked` type must be clocked.
#[allow(dead_code)] // Remove once all UARTs peripherals are implemented
pub struct Unclocked {
    pub(crate) uart1: pac::LPUART1,
    pub(crate) uart2: pac::LPUART2,
    pub(crate) uart3: pac::LPUART3,
    pub(crate) uart4: pac::LPUART4,
    pub(crate) uart5: pac::LPUART5,
    pub(crate) uart6: pac::LPUART6,
    pub(crate) uart7: pac::LPUART7,
    pub(crate) uart8: pac::LPUART8,
}
impl Unclocked {
    /// Enable all clocks for the UART peripherals. Returns a collection
    /// of UART peripherals.
    pub fn clock(
        self,
        ccm: &mut ccm::Handle,
        clock_select: ccm::uart::ClockSelect,
        prescalar: ccm::uart::PrescalarSelect,
    ) -> UARTs {
        let (ccm, _) = ccm.raw();

        //
        // See table 13-4 for clock gating registers
        //

        // -----------------------------------------
        // Disable clocks before modifying selection
        ccm.ccgr5.modify(|_, w| unsafe {
            w.cg12()
                .bits(0b00) // UART1
                .cg13()
                .bits(0b00) // UART7
        });
        ccm.ccgr0.modify(|_, w| unsafe {
            w.cg14()
                .bits(0b00) // UART2
                .cg6()
                .bits(0b00) // UART3
        });
        ccm.ccgr1.modify(|_, w| unsafe { w.cg12().bits(0b00) }); // UART4
        ccm.ccgr3.modify(|_, w| unsafe {
            w.cg1()
                .bits(0b00) // UART5
                .cg3()
                .bits(0b00) // UART6
        });
        ccm.ccgr6.modify(|_, w| unsafe { w.cg7().bits(0b00) }); // UART8

        // -----------------------------------------

        // -------------------------
        // Select clocks & prescalar
        ccm.cscdr1.write(|w| {
            w.uart_clk_sel()
                .variant(clock_select.into())
                .uart_clk_podf()
                .variant(prescalar)
        });
        // -------------------------

        // -------------
        // Enable clocks
        ccm.ccgr5.modify(|_, w| unsafe {
            w.cg12()
                .bits(0b11) // UART1
                .cg13()
                .bits(0b11) // UART7
        });
        ccm.ccgr0.modify(|_, w| unsafe {
            w.cg14()
                .bits(0b11) // UART2
                .cg6()
                .bits(0b11) // UART3
        });
        ccm.ccgr1.modify(|_, w| unsafe { w.cg12().bits(0b11) }); // UART4
        ccm.ccgr3.modify(|_, w| unsafe {
            w.cg1()
                .bits(0b11) // UART5
                .cg3()
                .bits(0b11) // UART6
        });
        ccm.ccgr6.modify(|_, w| unsafe { w.cg7().bits(0b11) }); // UART8

        // -------------

        let effective_clock = ccm::Frequency::from(clock_select) / ccm::Divider::from(prescalar);
        UARTs {
            uart1: Uninit::new(effective_clock, self.uart1),
            uart2: Uninit::new(effective_clock, self.uart2),
            uart3: Uninit::new(effective_clock, self.uart3),
            uart4: Uninit::new(effective_clock, self.uart4),
            uart5: Uninit::new(effective_clock, self.uart5),
            uart6: Uninit::new(effective_clock, self.uart6),
            uart7: Uninit::new(effective_clock, self.uart7),
            uart8: Uninit::new(effective_clock, self.uart8),
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
///
/// Call `read()` or `write()` to transmit bytes.
pub struct UART<M: module::Module> {
    reg: M::Reg,
    effective_clock: ccm::Frequency,
    _module: PhantomData<M>,
}

/// Parity selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    /// Even parity (the 'E' in 8E1, for example)
    Even,
    /// Odd parity (the 'O' in 8O1, for example)
    Odd,
}

impl Parity {
    fn bit(self) -> bool {
        self == Parity::Odd
    }
}

impl<M> UART<M>
where
    M: module::Module,
{
    fn start(
        reg: M::Reg,
        effective_clock: ccm::Frequency,
        baud: u32,
    ) -> Result<Self, ccm::uart::TimingsError> {
        let mut uart = UART {
            reg,
            effective_clock,
            _module: PhantomData,
        };
        uart.set_baud(baud)?;
        // Compiler can't infer type...?
        let reg: &M::Reg = &uart.reg;
        reg.ctrl.modify(|_, w| w.te().set_bit().re().set_bit());
        Ok(uart)
    }

    /// Specify parity bit settings. If there is no parity, use `None`.
    ///
    /// Calling this method will temporarily disable the peripheral,
    /// flusing all data from all FIFOs.
    pub fn set_parity(&mut self, parity: Option<Parity>) {
        self.while_disabled(|this| {
            this.reg.ctrl.modify(|_, w| {
                w.pe()
                    .bit(parity.is_some())
                    .m()
                    .bit(parity.is_some())
                    .pt()
                    .bit(parity.map(|p| p.bit()).unwrap_or(false))
            });
        });
    }

    /// Reverse the polarity of received data, affecting all data bits, start
    /// and stop bits, and polarity bits.
    ///
    /// The default inversion state is `false`. Note that calling this method
    /// will temporarily disable the peripheral, flusing all data from all FIFOs.
    pub fn set_rx_inversion(&mut self, inverted: bool) {
        self.while_disabled(|this| {
            this.reg.stat.modify(|_, w| w.rxinv().bit(inverted));
        });
    }

    /// Reverse the polarity of transferred data, affecting all data bits,
    /// start and stop bits, and polarity bits.
    ///
    /// The default inversion state is `false`. Note that calling this method
    /// will temporarily disable the peripheral, flusing all data from all FIFOs.
    pub fn set_tx_inversion(&mut self, inverted: bool) {
        self.while_disabled(|this| {
            this.reg.ctrl.modify(|_, w| w.txinv().bit(inverted));
        });
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
    /// the *both* TX and RX FIFOs.
    pub fn set_tx_fifo(&mut self, size: Option<core::num::NonZeroU8>) -> u8 {
        self.while_disabled(|this| {
            if let Some(requested_size) = size {
                // Maximum TX FIFO size supported by this device
                let max_size = 1 << this.reg.param.read().txfifo().bits();
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

    /// Enable or disable the RX FIFO. The maximum size of the FIFO is based on
    /// the underlying hardware. An iMXRT1062's RX FIFO is 4 bytes.
    ///
    /// Calling this method temporarily disables the peripheral, flusing all data
    /// from *both* TX and RX FIFOs.
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
    ///
    /// Calling this method temporarily disables the peripheral, flusing all data
    /// from *both* TX and RX FIFOs.
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

    /// Enable the receiver interrupt associated with this UART
    ///
    /// The interrupt will trigger when there are at least `watermark` number of
    /// bytes in the RX FIFO. Returns the maximum-allowable watermark level that
    /// was set in hardware. A watermark of `Some(0)` means that we should interrupt
    /// as soon as a byte is read.
    ///
    /// If the watermark is greater than 0, ensure that you call `set_rx_fifo` before this
    /// method. Otherwise, the return will be 0 despite the supplied watermark.
    ///
    /// Disable receiver interrupt by setting `watermark` to `None`. The return is always 0
    /// when disabling the receiver interrupt.
    pub fn set_receiver_interrupt(&mut self, watermark: Option<u8>) -> u8 {
        self.while_disabled(|this| {
            if let Some(watermark) = watermark {
                let rx_fifo_size = if this.reg.fifo.read().rxfe().bit_is_set() && watermark > 0 {
                    // Use the FIFO watermark to define interrupt frequency.
                    let max_size = 1 << this.reg.param.read().rxfifo().bits();
                    let fifo_size = max_size.min(watermark);
                    this.reg.water.modify(|_, w| unsafe {
                        // Safety: see justification in set_tx_fifo
                        w.rxwater().bits(fifo_size)
                    });
                    fifo_size
                } else {
                    // User has not enable the RX FIFO, or the watermark is zero.
                    0
                };
                this.reg.ctrl.modify(|_, w| w.rie().set_bit());
                rx_fifo_size
            } else {
                this.reg.ctrl.modify(|_, w| w.rie().clear_bit());
                0
            }
        })
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

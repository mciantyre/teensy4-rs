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
        Ok(uart)
    }

    pub fn set_baud(&mut self, baud: u32) -> Result<(), ccm::uart::TimingsError> {
        let timings = ccm::uart::timings(self.effective_clock, baud)?;
        self.reg
            .ctrl
            .modify(|_, w| w.te().clear_bit().re().clear_bit());
        self.reg.baud.modify(|_, w| unsafe {
            w.osr()
                .bits(timings.osr)
                .sbr()
                .bits(timings.sbr)
                .bothedge()
                .bit(timings.both_edge)
        });
        self.reg.ctrl.modify(|_, w| w.te().set_bit().re().set_bit());
        Ok(())
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
            return Err(nb::Error::WouldBlock);
        }
        Ok(())
    }
}

//! USB I/O support for nearly all of the USB examples
//!
//! This module supports a minimum example of how to use
//! the BSP's USB subsystem. Feel free to copy this module
//! into your own project.

// A single example might not use all functions in this module.
#![allow(dead_code)]

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use bsp::hal::ral::usb::USB1;
use bsp::interrupt;
use teensy4_bsp as bsp;

/// Initialize the USB logging system, and prepares the
/// USB ISR with the poller
///
/// When `init` returns, the USB interrupt will be enabled,
/// and the host may begin to interface the device.
/// You should only call this once.
///
/// # Panics
///
/// Panics if the imxrt-ral USB1 instance is already taken.
pub fn init() -> Result<bsp::usb::Reader, bsp::usb::Error> {
    let inst = USB1::take().unwrap();
    bsp::usb::init(inst, Default::default()).map(|(poller, reader)| {
        setup(poller);
        reader
    })
}

/// Split the USB logging system, and prepares the
/// USB ISR with the poller
///
/// When `split` returns, the USB interrupt will be enabled,
/// and the host may begin to interface the device.
/// You should only call this once.
///
/// # Panics
///
/// Panics if the imxrt-ral USB1 instance is already taken.
pub fn split() -> Result<(bsp::usb::Reader, bsp::usb::Writer), bsp::usb::Error> {
    let inst = USB1::take().unwrap();
    bsp::usb::split(inst).map(|(poller, reader, writer)| {
        setup(poller);
        (reader, writer)
    })
}

/// Setup the USB ISR with the USB poller
fn setup(poller: bsp::usb::Poller) {
    static POLLER: Mutex<RefCell<Option<bsp::usb::Poller>>> = Mutex::new(RefCell::new(None));

    #[cortex_m_rt::interrupt]
    fn USB_OTG1() {
        cortex_m::interrupt::free(|cs| {
            POLLER
                .borrow(cs)
                .borrow_mut()
                .as_mut()
                .map(|poller| poller.poll());
        });
    }

    cortex_m::interrupt::free(|cs| {
        *POLLER.borrow(cs).borrow_mut() = Some(poller);
        // Safety: invoked in a critical section that also prepares the ISR
        // shared memory. ISR memory is ready by the time the ISR runs.
        unsafe { cortex_m::peripheral::NVIC::unmask(bsp::interrupt::USB_OTG1) };
    });
}

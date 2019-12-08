//! We took some assumptions:
//!
//! - SYSTICK and delay implementation is very naive. Do not run for 49
//!   continuous days, or risk a millisecond counter wrap-around.

#![no_std]

pub use imxrt1060_hal as hal;
pub use teensy4_rt as rt;
use teensy4_usb_sys as usbsys;

pub use hal::pac::interrupt;
use rt::exception;
pub use usbsys::serial_write;
pub type LED = hal::gpio::IO03<hal::gpio::GPIO7, hal::gpio::Output>;

pub use hal::ccm::CCM;
pub use hal::pac::PIT;
pub use hal::pac::SYST;

pub struct Peripherals {
    pub led: LED,
    pub ccm: hal::ccm::CCM,
    pub pit: hal::pit::PIT<hal::pit::Unclocked>,
}

// See Section 12.3.2.1 of the reference manual. The note
// explains that the 24MHz clock is divided down to 100KHz
// before reaching SYSTICK.
const SYSTICK_EXT_FREQ: u32 = 100_000;

impl Peripherals {
    pub fn take() -> Option<Self> {
        let p = hal::Peripherals::take()?;
        Some(Peripherals::new(p))
    }

    fn new(mut p: hal::Peripherals) -> Peripherals {
        p.systick.disable_counter();
        p.systick
            .set_clock_source(cortex_m::peripheral::syst::SystClkSource::External);
        p.systick.set_reload((SYSTICK_EXT_FREQ / 1000) - 1);
        p.systick.clear_current();
        p.systick.enable_counter();
        p.systick.enable_interrupt();
        unsafe {
            usbsys::usb_pll_start();
            usbsys::usb_init();
            cortex_m::peripheral::NVIC::unmask(interrupt::USB_OTG1);
        }
        Peripherals {
            led: {
                let pad = p.iomuxc.gpio_b0_03;
                hal::gpio::IO03::gpio2(pad).fast(&mut p.iomuxc.gpr).output()
            },
            ccm: p.ccm,
            pit: p.pit,
        }
    }
}

#[no_mangle]
static mut systick_millis_count: u32 = 0;

// /// TODO(mciantyre) Not this...
#[no_mangle]
pub extern "C" fn delay(millis: u32) {
    if 0 == millis {
        return;
    }
    let start = unsafe { core::ptr::read_volatile(&systick_millis_count) };
    let target = start + millis;
    loop {
        let count = unsafe { core::ptr::read_volatile(&systick_millis_count) };
        if count >= target {
            return;
        }
    }
}

#[rt::interrupt]
fn USB_OTG1() {
    unsafe {
        usbsys::isr();
    }
}

#[rt::exception]
fn SysTick() {
    unsafe {
        let ms = core::ptr::read_volatile(&systick_millis_count);
        let ms = ms.wrapping_add(1);
        core::ptr::write_volatile(&mut systick_millis_count, ms);
    }
}

/// TODO(mciantyre) define a better yield
#[no_mangle]
fn r#yield() {
    cortex_m::asm::delay(1024);
}

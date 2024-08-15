//! Send messages from one LPSPI peripheral to another.
//!
//! Connect lpspi3 and lpspi4 like this:
//!
//! pin 11 (lpspi4 SDO) => pin 1 (lpspi3 SDI)
//! pin 12 (lpspi4 SDI) => pin 26 (lpspi3 SDO)
//! pin 13 (lpspi4 SCK) => pin 27 (lpspi3 SCK)
//! pin 10 (lpspi4 CD)  => pin 0 (lpspi3 CS)
//!
//! Despite targeting the Teensy 4.0, this starter code
//! should also work on the Teensy 4.1 and Teensy MicroMod.
//! You should eventually target your board! See inline notes.
//!
//! This template uses [RTIC v2](https://rtic.rs/2/book/en/)
//! for structuring the application.

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [KPP])]
mod app {
    use bsp::board;
    use teensy4_bsp as bsp;

    use imxrt_log as logging;

    use rtic_monotonics::systick::*;

    /// There are no resources shared across tasks.
    #[shared]
    struct Shared {}

    /// These resources are local to individual tasks.
    #[local]
    struct Local {
        lpspi3: board::Lpspi3<teensy4_bsp::pins::common::P1, teensy4_bsp::pins::common::P0>,

        /// Note: lpspi4 SCK is on pin 13 which collides with the Teensy 4/4.1 LED
        lpspi4: board::Lpspi4,

        /// A poller to control USB logging.
        poller: logging::Poller,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // Specify 't40', 't41', or 'tmm' (for MicroMod) depending on
        // which board you're using.
        let board::Resources {
            pins,
            usb,
            lpspi3,
            lpspi4,
            ..
        } = board::t40(cx.device);

        let poller = logging::log::usbd(usb, logging::Interrupts::Enabled).unwrap();

        let mut lpspi3 = board::lpspi(
            lpspi3,
            board::LpspiPins {
                sdo: pins.p26,
                sdi: pins.p1,
                sck: pins.p27,
                pcs0: pins.p0,
            },
            1_000_000,
        );
        // Turn lpspi3 into a peripheral (default is controller).
        lpspi3.disabled(|spi| {
            spi.set_peripheral_enable(true);
        });

        let lpspi4: board::Lpspi4 = board::lpspi(
            lpspi4,
            board::LpspiPins {
                sdo: pins.p11,
                sdi: pins.p12,
                sck: pins.p13,
                pcs0: pins.p10,
            },
            1_000_000,
        );

        Systick::start(
            cx.core.SYST,
            board::ARM_FREQUENCY,
            rtic_monotonics::create_systick_token!(),
        );

        lpspi3_reader::spawn().unwrap();
        lpspi4_writer::spawn().unwrap();

        (
            Shared {},
            Local {
                lpspi3,
                lpspi4,
                poller,
            },
        )
    }

    #[task(local = [lpspi4])]
    async fn lpspi4_writer(cx: lpspi4_writer::Context) {
        let mut buf: [u8; 4] = [0x50, 0, 0xff, 0x0a];
        loop {
            use embedded_hal::blocking::spi::Write;
            log::info!(
                "lpspi4 writing {:#04x} {:#04x} {:#04x} {:#04x}",
                buf[0],
                buf[1],
                buf[2],
                buf[3]
            );
            cx.local.lpspi4.write(&buf).unwrap_or_else(|error| {
                log::error!("spi write error: {error:?}");
            });
            buf[1] = buf[1].wrapping_add(1);
            Systick::delay((1000_u32).millis()).await;
        }
    }

    #[task(local = [lpspi3])]
    async fn lpspi3_reader(cx: lpspi3_reader::Context) {
        loop {
            if let Some(data) = cx.local.lpspi3.read_data() {
                log::info!("lpspi3 read data: {:#010x}", data);
            }
            Systick::delay((10_u32).millis()).await;
        }
    }

    #[task(binds = USB_OTG1, local = [poller])]
    fn usb_interrupt(cx: usb_interrupt::Context) {
        cx.local.poller.poll();
    }
}

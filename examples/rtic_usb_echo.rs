//! A USB example that echos data back to the USB host
//!
//! The example shows how you can use the `PollStatus`
//! from the `usb::poll` function to understand if data
//! is available from the host. Every echo causes the
//! LED to toggle.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [LPUART8])]
mod app {
    use super::copy;
    use bsp::hal::ral::usb::USB1;
    use teensy4_bsp as bsp;

    #[local]
    struct Local {
        led: bsp::Led,
        reader: bsp::usb::Reader,
        writer: bsp::usb::Writer,
        poller: bsp::usb::Poller,
    }

    #[shared]
    struct Shared {}

    /// Initialize the system
    ///
    /// - set the ARM clock speed
    /// - initialize the LED
    /// - initialize the USB reader and writer
    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        cx.device.ccm.pll1.set_arm_clock(
            bsp::hal::ccm::PLL1::ARM_HZ,
            &mut cx.device.ccm.handle,
            &mut cx.device.dcdc,
        );

        let pins = bsp::t40::into_pins(cx.device.iomuxc);
        let mut led = bsp::configure_led(pins.p13);
        led.set();

        let (poller, reader, writer) = bsp::usb::split(USB1::take().unwrap()).unwrap();

        (
            Shared {},
            Local {
                led,
                reader,
                writer,
                poller,
            },
            init::Monotonics(),
        )
    }

    /// This task drives the USB stack. If it detects that the host has
    /// received USB data, it schedules a task to echo back that data
    #[task(binds = USB_OTG1, local = [poller])]
    fn usb_otg1(cx: usb_otg1::Context) {
        let status = cx.local.poller.poll();
        if status.cdc_rx_complete() {
            echo::spawn().unwrap();
        }
    }

    /// Echo data back to the host
    #[task(local = [led, reader, writer])]
    fn echo(cx: echo::Context) {
        copy(cx.local.reader, cx.local.writer).unwrap();
        cx.local.led.toggle();
    }
}

/// Copy data from the reader to the writer
fn copy(
    reader: &mut bsp::usb::Reader,
    writer: &mut bsp::usb::Writer,
) -> Result<(), bsp::usb::Error> {
    let mut buffer: [u8; 256] = [0; 256];
    loop {
        let available = reader.read(&mut buffer)?;
        if available == 0 {
            break;
        }
        writer.write(&buffer[..available])?;
    }
    Ok(())
}

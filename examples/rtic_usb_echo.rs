//! A USB example that echos data back to the USB host
//!
//! The example shows how you can use the `PollStatus`
//! from the `usb::poll` function to understand if data
//! is available from the host. Every echo causes the
//! LED to toggle.

#![no_std]
#![no_main]

use bsp::hal::ral::usb::USB1;
use teensy4_bsp as bsp;
use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true)]
const APP: () = {
    struct Resources {
        led: bsp::Led,
        reader: bsp::usb::Reader,
        writer: bsp::usb::Writer,
        poller: bsp::usb::Poller,
    }

    /// Initialize the system
    ///
    /// - set the ARM clock speed
    /// - initialize the LED
    /// - initialize the USB reader and writer
    #[init]
    fn init(mut cx: init::Context) -> init::LateResources {
        cx.device.ccm.pll1.set_arm_clock(
            bsp::hal::ccm::PLL1::ARM_HZ,
            &mut cx.device.ccm.handle,
            &mut cx.device.dcdc,
        );

        let pins = bsp::t40::into_pins(cx.device.iomuxc);
        let mut led = bsp::configure_led(pins.p13);
        led.set();

        let (poller, reader, writer) = bsp::usb::split(USB1::take().unwrap()).unwrap();

        init::LateResources {
            led,
            poller,
            reader,
            writer,
        }
    }

    /// This task drives the USB stack. If it detects that the host has
    /// received USB data, it schedules a task to echo back that data
    #[task(binds = USB_OTG1, spawn = [echo], resources = [poller])]
    fn usb_otg1(cx: usb_otg1::Context) {
        let status = cx.resources.poller.poll();
        if status.cdc_rx_complete() {
            cx.spawn.echo().unwrap();
        }
    }

    /// Echo data back to the host
    #[task(resources = [led, reader, writer])]
    fn echo(cx: echo::Context) {
        copy(cx.resources.reader, cx.resources.writer).unwrap();
        cx.resources.led.toggle();
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn LPUART8();
    }
};

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

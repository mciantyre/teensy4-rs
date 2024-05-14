//! Demonstrates a custom USB logging stack with defmt.
//!
//! As of version 0.5 of the BSP, the BSP doesn't include an internal
//! logging stack. You're responsible for sourcing or building your
//! own. This example shows one way to build your own. The benefits
//! of building your own stack is that you can easily include additional
//! USB functions into your USB device stack. However, this example
//! is more complex, since it needs to configure the USB stack itself.
//!
//! # Overview
//!
//! The example sets up a USB serial device to stream data to the host. It
//! uses defmt-bbq as the defmt logger. Once the host configures the USB
//! device, the example periodically checks the logging queue for frames,
//! it and sends them to the host.
//!
//! If the LED is blinking, the example should be running.
//!
//! # Building
//!
//! This example enviroment will automatically configure defmt to log at
//! a specific level. See the Cargo configuration at .cargo/config.toml for
//! that log level.
//!
//! You otherwise build this example like any other example. See the README
//! in this directory for more information.
//!
//! # Monitoring the output
//!
//! Install [`defmt-print`]. Make sure to install a version that works with
//! the defmt version used in this firmare.
//!
//! `defmt-print`: https://crates.io/crates/defmt-print
//!
//! Then, pipe bytes from your target into `defmt-print`. Here's one way to
//! do that on a *nix system:
//!
//! ```text
//! cat /dev/${YOUR_SERIAL_DEVICE} | defmt-print -e target/thumbv7em-none-eabihf/release/examples/rtic_defmt_usb_log
//! ```

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [KPP])]
mod app {
    use bsp::{
        board,
        hal::usbd::{
            gpt::{Instance::Gpt0, Mode},
            BusAdapter, EndpointMemory, EndpointState, Speed,
        },
    };
    use teensy4_bsp as bsp;

    use usb_device::{
        bus::UsbBusAllocator,
        device::{UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
    };
    use usbd_serial::SerialPort;

    use defmt_bbq::DefmtConsumer;

    use rtic_monotonics::systick::*;

    /// We're intentionally using a full-speed device instead of a high-speed
    /// device. The full-speed device has better support in the usb-device
    /// ecosystem (in terms of packages and host support), and we don't need a
    /// high-speed device for this efficient logging.
    const SPEED: Speed = Speed::LowFull;
    /// Looking for USB devices on your host? Search for this VID and PID.
    const VID_PID: UsbVidPid = UsbVidPid(0x5824, 0x27dd);
    /// You could also look for this product ID.
    const PRODUCT: &str = "teensy4-bsp-example";

    #[local]
    struct Local {
        /// Use this to send data to your host.
        usb_class: SerialPort<'static, BusAdapter>,
        /// Use this to manage the USB device stack.
        usb_device: UsbDevice<'static, BusAdapter>,
        /// This queue endpoint has defmt frames to send
        /// to the host.
        defmt_consumer: DefmtConsumer,
        /// For periodically signaling activity.
        led: board::Led,
    }

    #[shared]
    struct Shared {}

    #[init(local = [
        // This allocation is used for USB endpoint I/O, and it's shared
        // across all USB endpoints.
        ep_memory: EndpointMemory<1024> = EndpointMemory::new(),
        // This allocation is for the endpoints themselves. It's sized for
        // all possible endpoints, even if we're not using them in this
        // example.
        ep_state: EndpointState = EndpointState::max_endpoints(),
        // And this is the USB bus itself. It's set to Some(...) below.
        usb_bus: Option<UsbBusAllocator<BusAdapter>> = None,
    ])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            usb,
            pins,
            mut gpio2,
            ..
        } = board::t40(cx.device);
        let led = board::led(&mut gpio2, pins.p13);

        // Set up the USB bus with the components provided by the board.
        let bus_adapter = BusAdapter::with_speed(usb, cx.local.ep_memory, cx.local.ep_state, SPEED);
        // We need USB interrupts to activate. Otherwise, we won't be able to respond
        // to the host.
        bus_adapter.set_interrupts(true);
        // We want periodic interrupts in order to check the defmt queue. The USB
        // device has its own periodic timers we can use for this purpose.
        bus_adapter.gpt_mut(Gpt0, |gpt| {
            gpt.stop();
            gpt.clear_elapsed();
            gpt.set_interrupt_enabled(true);
            gpt.set_mode(Mode::Repeat);
            gpt.set_load(10_000); // microseconds.
            gpt.reset();
            gpt.run();
        });

        let usb_bus = cx.local.usb_bus.insert(UsbBusAllocator::new(bus_adapter));
        let usb_class = SerialPort::new(usb_bus);
        let usb_device = UsbDeviceBuilder::new(usb_bus, VID_PID)
            .product(PRODUCT)
            .device_class(usbd_serial::USB_CLASS_CDC)
            .build();

        let defmt_consumer = defmt_bbq::init().unwrap();

        // Set up a system timer for our software task.
        {
            Systick::start(
                cx.core.SYST,
                board::ARM_FREQUENCY,
                rtic_monotonics::create_systick_token!(),
            );
        }

        // Schedule that software task to run.
        make_logs::spawn().unwrap();

        // If the LED turns on, we've made it past init.
        led.set();

        (
            Shared {},
            Local {
                usb_class,
                usb_device,
                defmt_consumer,
                led,
            },
        )
    }

    /// This task periodically logs data.
    ///
    /// You won't see all the log levels until you configure your build. See the
    /// top-level docs for more information.
    #[task(local = [led])]
    async fn make_logs(cx: make_logs::Context) {
        let make_logs::LocalResources { led, .. } = cx.local;

        let mut counter = 0u32;
        loop {
            led.toggle();
            Systick::delay(250.millis()).await;

            defmt::println!("Hello from defmt! The count is {=u32}", counter);
            defmt::trace!("TRACE: {=u32}", counter);

            if counter % 3 == 0 {
                defmt::debug!("DEBUG: {=u32}", counter);
            }

            if counter % 5 == 0 {
                defmt::info!("INFO: {=u32}", counter);
            }

            if counter % 7 == 0 {
                defmt::warn!("WARN: {=u32}", counter);
            }

            if counter % 31 == 0 {
                defmt::error!("ERROR: {=u32}", counter);
            }

            counter = counter.wrapping_add(1);
        }
    }

    /// This task runs when the USB1 interrupt activates. It removes
    /// defmt data from the queue and sends it to a USB host.
    #[task(binds = USB_OTG1, local = [
        usb_class, usb_device, defmt_consumer,
        configured: bool = false,
    ])]
    fn usb_interrupt(cx: usb_interrupt::Context) {
        let usb_interrupt::LocalResources {
            usb_class,
            usb_device,
            defmt_consumer,
            configured,
            ..
        } = cx.local;

        // If we're here because the timer elapsed, we should clear
        // that status.
        usb_device.bus().gpt_mut(Gpt0, |gpt| {
            while gpt.is_elapsed() {
                gpt.clear_elapsed();
            }
        });

        // Do we have USB packets to handle?
        if usb_device.poll(&mut [usb_class]) {
            // Are we newly configured?
            if usb_device.state() == UsbDeviceState::Configured {
                // Is this our first configuration? See the imxrt-usbd API
                // documentation for more information on this requirement.
                if !*configured {
                    usb_device.bus().configure();
                }
                *configured = true;
            } else {
                // We might have lost our configuration!
                *configured = false;
            }
        }

        // We can only touch the class once we're configured...
        if *configured {
            // Remove bytes from the defmt queue and send them to the host.
            while let Ok(grant) = defmt_consumer.read() {
                if let Ok(written) = usb_class.write(&grant) {
                    grant.release(written);
                } else {
                    break;
                }
            }
        }
    }
}

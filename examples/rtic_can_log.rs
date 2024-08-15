//! Demonstrates how to use a FlexCAN peripheral..
//!
//! This example requires:
//!
//! - The `rt` feature to be enabled.
//! - a Can transceiver (such as the Texas Instruments SN65HVD230).
//!
//! Pinout:
//!
//! - Teensy 4 Pin 22 (CAN1 TX) to TX pin of CAN Transceiver
//! - Teensy 4 Pin 23 (CAN2 RX) to RX pin of CAN Transceiver
//!
//!

#![no_std]
#![no_main]

use teensy4_panic as _;

// Type aliases for the Queue we want to use.
// type Ty = u8;
// const CAP: usize = 256;
// type Queue = heapless::spsc::Queue<Ty, { CAP }>;
// type Consumer = heapless::spsc::Consumer<'static, Ty, { CAP }>;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [KPP])]
mod app {
    use bsp::board;
    use teensy4_bsp as bsp;

    use imxrt_log as logging;

    use rtic_monotonics::systick::{self, *};

    #[local]
    struct Local {
        /// For driving the logging endpoint.
        poller: logging::Poller,
        /// For periodically signaling activity.
        led: board::Led,
    }

    #[shared]
    struct Shared {
        // CAN interface on the board
        can: board::Flexcan1,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            usb,
            pins,
            flexcan1,
            mut gpio2,
            ..
        } = board::t41(cx.device);
        let led = board::led(&mut gpio2, pins.p13);

        // Set up the logging system.
        //
        // There's various ways to control log levels at build- and run-time.
        // See the imxrt-log documentation for more information. This example
        // doesn't demonstrate any of that.
        let poller = logging::log::usbd(usb, logging::Interrupts::Enabled).unwrap();

        let mut can = board::flexcan(flexcan1, pins.p22, pins.p23);
        can.set_baud_rate(125_000);
        can.set_max_mailbox(16);
        can.disable_fifo();
        // Set up a system timer for our software task.
        {
            Systick::start(
                cx.core.SYST,
                board::ARM_FREQUENCY,
                rtic_monotonics::create_systick_token!(),
            );
        }

        blink::spawn().unwrap();

        run_can_rx::spawn().unwrap();
        run_can_tx::spawn().unwrap();
        // If the LED turns on, we've made it past init.
        led.set();

        (Shared { can }, Local { poller, led })
    }

    #[task(shared = [can])]
    async fn run_can_rx(cx: run_can_rx::Context) {
        let run_can_rx::SharedResources { mut can, .. } = cx.shared;
        loop {
            // read all available mailboxes for any available frames
            if let Some(data) = can.lock(|can| can.read_mailboxes()) {
                log::info!("RX: MB{} - {:?}", data.mailbox_number, data.frame);
            }

            Systick::delay(1.millis()).await;
        }
    }

    #[task(shared = [can])]
    async fn run_can_tx(cx: run_can_tx::Context) {
        let run_can_tx::SharedResources { mut can, .. } = cx.shared;
        // create a `Frame` with `StandardID` 0x00
        // and `Data` [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]
        let id = imxrt_hal::can::Id::from(imxrt_hal::can::StandardId::new(0x03).unwrap());
        let data: [u8; 1] = [0x03];
        let frame = imxrt_hal::can::Frame::new_data(id, data);

        loop {
            // read all available mailboxes for any available frames
            can.lock(|can| can.transmit(&frame));
            Systick::delay(1000.millis()).await;
        }
    }

    /// This task runs when the USB1 interrupt activates.
    /// Simply poll the logger to control the logging process.
    #[task(binds = USB_OTG1, local = [poller])]
    fn usb_interrupt(cx: usb_interrupt::Context) {
        cx.local.poller.poll();
    }

    #[task(local = [led])]
    async fn blink(cx: blink::Context) {
        loop {
            // Toggle the LED.
            cx.local.led.toggle();
            Systick::delay(1000.millis()).await;
        }
    }
}

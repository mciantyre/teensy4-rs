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

    // FIXME: need these?

    // use crate::{usb_io, Consumer, Queue};
    // use crate::Consumer;
    // use crate::Queue;
    // use embedded_hal::digital::v2::OutputPin;
    // use bsp::hal::iomuxc::consts::U1;

    // use dwt_systick_monotonic::{fugit::ExtU32, DwtSystick};

    // #[monotonic(binds = SysTick, default = true)]
    // type MyMono = DwtSystick<{ bsp::hal::ccm::PLL1::ARM_HZ }>;

    #[local]
    struct Local {
        /// For driving the logging endpoint.
        poller: logging::Poller,
        /// For periodically signaling activity.
        led: board::Led,
        // q_rx: Consumer,
        // blink_count: u32,
    }

    #[shared]
    struct Shared {
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
        // can.set_fifo_interrupt(true);
        // can.set_fifo_accept_all();
        // Set up a system timer for our software task.
        {
            Systick::start(
                cx.core.SYST,
                board::ARM_FREQUENCY,
                rtic_monotonics::create_systick_token!(),
            );
        }

        // Schedule that software task to run.
        // make_logs::spawn().unwrap();

        // The queue used for buffering bytes.
        // let (_q_tx, q_rx) = cx.local.queue.split();

        //        let mut peripherals = bsp::hal::Peripherals::take().unwrap();
        //
        //        let (can1_builder, _) = peripherals.can.clock(
        //            &mut peripherals.ccm.handle,
        //            bsp::hal::ccm::can::ClockSelect::OSC,
        //            bsp::hal::ccm::can::PrescalarSelect::DIVIDE_1,
        //        );
        //
        //        let mut can1 = can1_builder.build(pins.p22, pins.p23);
        //
        //        can1_init::spawn_after(1_u32.secs()).unwrap();

        // Schedule the first blink.
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

    /// This task periodically logs data.
    ///
    /// You won't see all the log levels until you configure your build. See the
    /// top-level docs for more information.
    // #[task(local = [led])]
    // async fn make_logs(cx: make_logs::Context) {
    //     let make_logs::LocalResources { led, .. } = cx.local;

    //     let mut counter = 0u32;
    //     loop {
    //         led.toggle();
    //         Systick::delay(250.millis()).await;

    //         log::trace!("TRACE: {}", counter);

    //         if counter % 3 == 0 {
    //             log::debug!("DEBUG: {}", counter);
    //         }

    //         if counter % 5 == 0 {
    //             log::info!("INFO: {}", counter);
    //         }

    //         if counter % 7 == 0 {
    //             log::warn!("WARN: {}", counter);
    //         }

    //         if counter % 31 == 0 {
    //             log::error!("ERROR: {}", counter);
    //         }

    //         counter = counter.wrapping_add(1);
    //     }
    // }

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

    //    #[task(shared = [can1])]
    //    async fn can1_init(mut cx: can1_init::Context) {
    //        cx.shared.can1.lock(|can1| {
    //            can1.set_baud_rate(1_000_000);
    //            can1.set_max_mailbox(16);
    //            can1.enable_fifo();
    //            can1.set_fifo_interrupt(true);
    //            can1.set_fifo_accept_all();
    //            can1.print_registers();
    //        });
    //        can1::spawn_after(100_u32.millis()).unwrap();
    //    }
    //
    //    #[task(shared = [can1])]
    //    async fn can1(mut cx: can1::Context) {
    //        let data: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    //        let id = bsp::hal::can::Id::from(bsp::hal::can::StandardId::new(0x00).unwrap());
    //        let frame = bsp::hal::can::Frame::new_data(id, data);
    //        cx.shared.can1.lock(|can1| {
    //            can1.transmit(&frame).ok();
    //        });
    //        can1::spawn_after(100_u32.millis()).unwrap();
    //    }
    //
    //    #[task(binds = CAN1, shared = [can1],)]
    //    fn can1_int(mut cx: can1_int::Context) {
    //        cx.shared.can1.lock(|can1| {
    //            match can1.handle_interrupt() {
    //                Some(f) => log::info!("Rx: {:?}", &f),
    //                None => { },
    //            };
    //        });
    //    }
}

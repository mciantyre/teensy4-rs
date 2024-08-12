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
mod usb_io;

// Type aliases for the Queue we want to use.
type Ty = u8;
const CAP: usize = 256;
type Queue = heapless::spsc::Queue<Ty, { CAP }>;
type Consumer = heapless::spsc::Consumer<'static, Ty, { CAP }>;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [LPUART8])]
mod app {
    use crate::{usb_io, Consumer, Queue};
    use embedded_hal::digital::v2::OutputPin;
    use teensy4_bsp as bsp;
    use bsp::hal::iomuxc::consts::U1;

    use dwt_systick_monotonic::{fugit::ExtU32, DwtSystick};

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<{ bsp::hal::ccm::PLL1::ARM_HZ }>;

    #[local]
    struct Local {
        led: bsp::Led,
        q_rx: Consumer,
        blink_count: u32,
    }

    #[shared]
    struct Shared {
        can1: bsp::hal::can::CAN<U1>,
    }

    #[init(local = [
        queue: Queue = heapless::spsc::Queue::new(),
    ])]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;
        let systick = cx.core.SYST;

        usb_io::init().unwrap();

        let mono = DwtSystick::new(&mut dcb, dwt, systick, bsp::hal::ccm::PLL1::ARM_HZ);

        cx.device.ccm.pll1.set_arm_clock(
            bsp::hal::ccm::PLL1::ARM_HZ,
            &mut cx.device.ccm.handle,
            &mut cx.device.dcdc,
        );

        let pins = bsp::pins::t40::from_pads(cx.device.iomuxc);

        let (can1_builder, _) = peripherals.can.clock(
            &mut peripherals.ccm.handle,
            bsp::hal::ccm::can::ClockSelect::OSC,
            bsp::hal::ccm::can::PrescalarSelect::DIVIDE_1,
        );
    
        let mut can1 = can1_builder.build(pins.p22, pins.p23);

        // The queue used for buffering bytes.
        let (_q_tx, q_rx) = cx.local.queue.split();

        // LED setup.
        let mut led = bsp::configure_led(pins.p13);
        led.set_high().unwrap();

        // Schedule the first blink.
        blink::spawn_after(1_u32.secs()).unwrap();

        can1_init::spawn_after(1_u32.secs()).unwrap();

        (
            Shared { can1 },
            Local {
                led,
                q_rx,
                blink_count: 0,
            },
            init::Monotonics(mono),
        )
    }

    #[task(local = [led, q_rx, blink_count])]
    fn blink(cx: blink::Context) {
        if cx.local.q_rx.ready() {
            let mut buffer = [0u8; 256];
            for elem in buffer.iter_mut() {
                *elem = match cx.local.q_rx.dequeue() {
                    None => break,
                    Some(b) => b,
                };
            }
            let s = core::str::from_utf8(&buffer).unwrap();
            log::info!("read: {}", s);
        }

        // Toggle the LED.
        cx.local.led.toggle();

        // Schedule the following blink.
        blink::spawn_after(100_u32.millis()).unwrap();
    }

    #[task(shared = [can1])]
    fn can1_init(mut cx: can1_init::Context) {
        cx.shared.can1.lock(|can1| {
            can1.set_baud_rate(1_000_000);
            can1.set_max_mailbox(16);
            can1.enable_fifo();
            can1.set_fifo_interrupt(true);
            can1.set_fifo_accept_all();
            can1.print_registers();
        });
        can1::spawn_after(100_u32.millis()).unwrap();
    }

    #[task(shared = [can1])]
    fn can1(mut cx: can1::Context) {
        let data: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let id = bsp::hal::can::Id::from(bsp::hal::can::StandardId::new(0x00).unwrap());
        let frame = bsp::hal::can::Frame::new_data(id, data);
        cx.shared.can1.lock(|can1| {
            can1.transmit(&frame).ok();
        });
        can1::spawn_after(100_u32.millis()).unwrap();
    }

    #[task(binds = CAN1, shared = [can1],)]
    fn can1_int(mut cx: can1_int::Context) {
        cx.shared.can1.lock(|can1| {
            match can1.handle_interrupt() {
                Some(f) => log::info!("Rx: {:?}", &f),
                None => { },
            };
        });
    }
}

//! An adaptation of the `rtic_uart_log.rs` example that demonstrates using the DMA controller
//! (Direct Memory Access) to improve performance when logging via UART. Notice the additional
//! hardware task executed via the `DMA7_DMA23` interrupt. This interrupt occurs when the DMA
//! controller has finished moving the submitted data to the UART peripheral buffer.
//!
//! Learn more about DMA UART logging here: https://docs.rs/imxrt-uart-log.
//!
//! This example requires:
//!
//! - The `rtic` feature to be enabled.
//! - a serial to USB converter (tested with CP2102). The converter should be connected to pins 14
//! and 15. Pin 14 is teensy's TX and pin 15 is teensy's RX pin.
//!
//! Success criteria:
//!
//! - The on-board LED should blink once per second.
//! - On each blink, we receive a message from the teensy via the serial console (e.g. `screen`).
//! - When writing serial data from the console, the teensy should log when each call to the
//! interrupt hardware task occurs and prints the characters received as a utf8 string on each
//! blink.

#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::serial::Read;
use heapless::consts::U256;
use panic_halt as _;
use rtic::cyccnt::U32Ext;
use teensy4_bsp as bsp;

const PERIOD: u32 = bsp::hal::ccm::PLL1::ARM_HZ;
const BAUD: u32 = 115_200;
const TX_FIFO_SIZE: u8 = 4;

// Type aliases for the Queue we want to use.
type Ty = u8;
type Cap = U256;
type Queue = heapless::spsc::Queue<Ty, Cap>;
type Producer = heapless::spsc::Producer<'static, Ty, Cap>;
type Consumer = heapless::spsc::Consumer<'static, Ty, Cap>;

// The UART receiver.
type UartRx = bsp::hal::uart::Rx<bsp::hal::iomuxc::consts::U2>;

#[rtic::app(device = teensy4_bsp, monotonic = rtic::cyccnt::CYCCNT, peripherals = true)]
const APP: () = {
    struct Resources {
        led: bsp::LED,
        u_rx: UartRx,
        q_tx: Producer,
        q_rx: Consumer,
        dma_interrupt_count: u32,
    }

    #[init(schedule = [blink])]
    fn init(mut cx: init::Context) -> init::LateResources {
        init_delay();

        // Setup the clock for rtic scheduling.
        cx.device.ccm.set_mode(bsp::hal::ccm::ClockMode::Run);
        cx.core.DWT.enable_cycle_counter();
        cx.device.ccm.pll1.set_arm_clock(
            bsp::hal::ccm::PLL1::ARM_HZ,
            &mut cx.device.ccm.handle,
            &mut cx.device.dcdc,
        );

        let pins = bsp::t40::pins(cx.device.iomuxc);

        // DMA setup.
        let mut dma_channels = cx.device.dma.clock(&mut cx.device.ccm.handle);
        let channel = dma_channels[7].take().unwrap();

        // UART setup.
        let uarts = cx.device.uart.clock(
            &mut cx.device.ccm.handle,
            bsp::hal::ccm::uart::ClockSelect::OSC,
            bsp::hal::ccm::uart::PrescalarSelect::DIVIDE_1,
        );
        let mut uart = uarts.uart2.init(pins.p14, pins.p15, BAUD).unwrap();
        uart.set_tx_fifo(core::num::NonZeroU8::new(TX_FIFO_SIZE));
        uart.set_rx_fifo(true);
        uart.set_receiver_interrupt(Some(0));
        let (u_tx, u_rx) = uart.split();

        // Logging setup.
        imxrt_uart_log::dma::init(u_tx, channel, Default::default()).unwrap();

        // The queue used for buffering bytes.
        static mut Q: Queue = heapless::spsc::Queue(heapless::i::Queue::new());
        let (q_tx, q_rx) = unsafe { Q.split() };

        // LED setup.
        let mut led = bsp::configure_led(pins.p13);
        led.set_high().unwrap();

        // Schedule the first blink.
        cx.schedule.blink(cx.start + PERIOD.cycles()).unwrap();

        init::LateResources {
            led,
            u_rx,
            q_tx,
            q_rx,
            dma_interrupt_count: 0,
        }
    }

    #[task(resources = [led, q_rx, dma_interrupt_count], schedule = [blink])]
    fn blink(cx: blink::Context) {
        static mut TIMES: u32 = 0;
        *TIMES += 1;
        let plural = if *TIMES > 1 { "s" } else { "" };
        log::info!("`blink` called {} time{}", *TIMES, plural);
        log::info!(
            "DMA7_DMA23 interrupted {} times",
            cx.resources.dma_interrupt_count
        );

        if cx.resources.q_rx.ready() {
            let mut buffer = [0u8; 256];
            for elem in buffer.iter_mut() {
                *elem = match cx.resources.q_rx.dequeue() {
                    None => break,
                    Some(b) => b,
                };
            }
            let s = core::str::from_utf8(&buffer).unwrap();
            log::info!("read: {}", s);
        }

        // Toggle the LED.
        cx.resources.led.toggle();

        // Schedule the following blink.
        cx.schedule.blink(cx.scheduled + PERIOD.cycles()).unwrap();
    }

    #[task(binds = LPUART2, resources = [u_rx, q_tx])]
    fn lpuart2(cx: lpuart2::Context) {
        log::info!("LPUART2 interrupt task called!");
        while let Ok(b) = cx.resources.u_rx.read() {
            cx.resources.q_tx.enqueue(b).ok();
        }
    }

    #[task(binds = DMA7_DMA23, resources = [dma_interrupt_count])]
    fn dma7_dma23(cx: dma7_dma23::Context) {
        *cx.resources.dma_interrupt_count += 1;
        imxrt_uart_log::dma::poll();
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn LPUART8();
    }
};

// If we reach WFI on teensy 4.0 too quickly it seems to halt. Here we wait a short while in `init`
// to avoid this issue. The issue only appears to occur when rebooting the device (via the button),
// however there appears to be no issue when power cycling the device.
//
// TODO: Investigate exactly why this appears to be necessary.
fn init_delay() {
    for _ in 0..10_000_000 {
        core::sync::atomic::spin_loop_hint();
    }
}

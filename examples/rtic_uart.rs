//! A loopback device. Send characters, and you should see
//! the exact same characters sent back. The LED toggles for
//! every exchanged character.
//!
//! - Pin 14 is the Teensy's TX.
//! - Pin 15 is the Teensy's RX.
//!
//! Baud: 115200bps.

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true)]
mod app {
    use bsp::board;
    use bsp::hal::lpuart;
    use teensy4_bsp as bsp;

    #[local]
    struct Local {
        led: board::Led,
        lpuart2: board::Lpuart2,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board::Resources {
            pins,
            lpuart2,
            mut gpio2,
            ..
        } = board::t40(cx.device);
        let led = board::led(&mut gpio2, pins.p13);
        led.set();

        let mut lpuart2: board::Lpuart2 = board::lpuart(lpuart2, pins.p14, pins.p15, 115200);
        lpuart2.disable(|lpuart2| {
            lpuart2.disable_fifo(lpuart::Direction::Tx);
            lpuart2.disable_fifo(lpuart::Direction::Rx);
            lpuart2.set_interrupts(lpuart::Interrupts::RECEIVE_FULL);
            lpuart2.set_parity(None);
        });
        (Shared {}, Local { led, lpuart2 }, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = LPUART2, local = [led, lpuart2])]
    fn lpuart2_interrupt(cx: lpuart2_interrupt::Context) {
        use lpuart::Status;
        let lpuart2 = cx.local.lpuart2;
        let led = cx.local.led;

        let status = lpuart2.status();
        lpuart2.clear_status(Status::W1C);

        if status.contains(Status::RECEIVE_FULL) {
            loop {
                let data = lpuart2.read_data();
                if data.flags().contains(lpuart::ReadFlags::RXEMPT) {
                    break;
                }
                if lpuart2.status().contains(Status::TRANSMIT_EMPTY) {
                    lpuart2.write_byte(data.into());
                }
            }
            led.toggle();
        }
    }
}

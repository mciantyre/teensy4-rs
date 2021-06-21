//! Demonstrates using the SRTC, including setting time from USB serial.
//!
//! Flash this example to your Teensy, then send the current Unix time over the USB serial port.
//!
//! Send either just the seconds, e.g. `1601896065`, or seconds.nanos, e.g. `1601896065.068985800`.
//! The whole message must be sent at once; typing it will take the first digit as the time.
//!
//! On a GNU system, for example: `date +%s.%N >> /dev/ttyS5` (where `ttyS5` is the serial port
//! of the Teensy 4)
//!
//! On Windows, try pasting the Unix time into the serial window.
//!
//! On any host platform, use `SystemTime` to get the Unix time:
//!
//! ```no_run
//! use std::time::SystemTime;
//!
//! let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
//! let seconds = now.as_secs();
//! let ns = now.subsec_nanos();
//! let message = format!("{}.{:09}", seconds, ns);
//! ```
//!
//! and then send that message using a crate that provides support for serial port communication.

#![no_std]
#![no_main]

mod systick;
mod usb_io;

use bsp::hal::srtc::{micros_to_ticks, EnabledState, SRTC};
use bsp::usb;
use core::fmt::Write;
use cortex_m_rt as rt;
use teensy4_bsp as bsp;
use teensy4_panic as _;

#[rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    let mut systick = systick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let (mut reader, mut writer) = usb_io::split().unwrap();

    systick.delay_ms(2000);
    p.ccm
        .pll1
        .set_arm_clock(bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
    let srtc = match p.srtc.try_enable(&mut p.ccm.handle, 0, 0) {
        EnabledState::AlreadyCounting { srtc, seconds, .. } => {
            writeln!(writer, "SRTC already on, counting from {}", seconds).unwrap();
            srtc
        }
        EnabledState::SetTime(mut srtc) => {
            // SRTC wasn't on, request time from USB
            set_time_from_usb(&mut reader, &mut writer, &mut srtc);
            srtc
        }
    };
    let mut now = srtc.get();
    loop {
        writeln!(writer, "The current Unix time is {}", now).unwrap();
        while now == srtc.get() {}
        now = srtc.get();
    }
}

fn set_time_from_usb(reader: &mut usb::Reader, writer: &mut usb::Writer, rtc: &mut SRTC) {
    writeln!(writer, "Send a message containing the current Unix time.").unwrap();
    let mut buffer = [0; 24]; // 20 char message + CR LF + an extra 2
    let (seconds, ns) = loop {
        let count = reader.read(&mut buffer).unwrap();
        if count == 0 {
            continue;
        }
        if let Ok(message) = core::str::from_utf8(&buffer[..count]) {
            let mut split = message.trim_end().split('.');
            if let Some(seconds) = split.next() {
                if let Ok(seconds) = seconds.parse::<u32>() {
                    let ns = split.next().unwrap_or("0");
                    let ns = ns.parse::<u32>().unwrap_or(0);
                    break (seconds, ns);
                }
            }
        }
    };
    let us = ns / 1000;
    let ticks = micros_to_ticks(us);
    rtc.set(seconds, ticks);
    writeln!(writer, "Received time: {}.{:09}", seconds, ns).unwrap();
}

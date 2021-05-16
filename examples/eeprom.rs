//! EEPROM emulation
//!
//! Demonstrates how to read and write to an EEPROM address.
//! Success: the example doesn't panic.

#![no_std]
#![no_main]

use teensy4_panic as _;

use cortex_m_rt as rt;
use teensy4_bsp as bsp;

const PERIOD_MS: u32 = 1_000;
const EEPROM_RW_ADDRESS: usize = 42;
const EEPROM_PERSISTENCE_ADDRESS: usize = 137;
const EEPROM_SENTINEL: u8 = 78;

#[rt::entry]
fn main() -> ! {
    let p = bsp::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let pins = bsp::t40::into_pins(p.iomuxc);
    let mut led = bsp::configure_led(pins.p13);

    let mut eeprom = bsp::Eeprom::new().unwrap();
    if EEPROM_SENTINEL == eeprom.read_byte(EEPROM_PERSISTENCE_ADDRESS) {
        // Run this example at least once. Then, power cycle your Teensy4. You
        // should see this branch of code is hit (manifests as the LED turns on
        // immediately, instead of after one second.)
        led.set();
    }
    eeprom.write_byte(EEPROM_PERSISTENCE_ADDRESS, EEPROM_SENTINEL);

    let mut expected = 0;
    eeprom.write_byte(EEPROM_RW_ADDRESS, expected);
    loop {
        systick.delay(PERIOD_MS);
        led.toggle();

        let actual = eeprom.read_byte(EEPROM_RW_ADDRESS);
        assert_eq!(actual, expected);
        expected = expected.wrapping_add(1);
        eeprom.write_byte(EEPROM_RW_ADDRESS, expected);
    }
}

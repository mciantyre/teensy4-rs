//! Panic handler for Teensy 4 systems

#![no_std]

/// The LED
struct LED();

const GPIO2_BASE: u32 = 0x401BC000;

impl LED {
    /// Construct the LED
    ///
    /// When `new` returns, the LED will be ready to set, clear, and toggle.
    fn new() -> Self {
        const IOMUXC_SW_MUX_CTL_PAD_GPIO_B0_03: *mut u32 = 0x401F_8148 as *mut u32;
        const IOMUXC_SW_PAD_CTL_PAD_GPIO_B0_03: *mut u32 = 0x401F_8338 as *mut u32;
        const IOMUXC_GPR_GPR27: *mut u32 = 0x400A_C06C as *mut u32;

        const fn drive_strength_enable(dse: u32) -> u32 {
            (dse & 0x07) << 3
        }

        const GPIO2_GDIR: *mut u32 = (GPIO2_BASE + 0x04) as *mut u32;

        // Safety: this modifies IOMUXC memory that may be configured for another
        // peripheral. But, this is a panic handler, and we're not returning from
        // this state.
        unsafe {
            // Set the LED pad into Alt5
            IOMUXC_SW_MUX_CTL_PAD_GPIO_B0_03.write_volatile(5);
            // Increase drive strength, clearing all other fields
            IOMUXC_SW_PAD_CTL_PAD_GPIO_B0_03.write_volatile(drive_strength_enable(7));
            // Disable fast mode so that GPIO2 registers drive the pad
            IOMUXC_GPR_GPR27.write_volatile(IOMUXC_GPR_GPR27.read_volatile() & !(1 << 3));
            GPIO2_GDIR.write_volatile(GPIO2_GDIR.read_volatile() | (1 << 3))
        }

        LED()
    }

    /// Drive the LED high
    fn set(&mut self) {
        const GPIO2_DR_SET: *mut u32 = (GPIO2_BASE + 0x84) as *mut u32;
        // Safety: modify global peripheral memory, but no one will ever notice
        // since we don't return from the panic handler
        unsafe { GPIO2_DR_SET.write_volatile(1 << 3) };
    }

    /// Drive the LED low
    fn clear(&mut self) {
        const GPIO2_DR_CLEAR: *mut u32 = (GPIO2_BASE + 0x88) as *mut u32;
        // Safety: modify global peripheral memory, but no one will ever notice
        // since we don't return from the panic handler
        unsafe { GPIO2_DR_CLEAR.write_volatile(1 << 3) };
    }
}

fn delay(factor: u32) {
    for _ in 0..(factor * 50_000_000) {
        core::sync::atomic::spin_loop_hint();
    }
}

fn triple(led: &mut LED, factor: u32) {
    for _ in 0..3 {
        led.set();
        delay(factor);
        led.clear();
        delay(factor);
    }
}

fn s(led: &mut LED) {
    triple(led, 1);
}

fn o(led: &mut LED) {
    triple(led, 3);
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    let mut led = LED::new();
    loop {
        s(&mut led);
        o(&mut led);
        s(&mut led);
        delay(6);
    }
}

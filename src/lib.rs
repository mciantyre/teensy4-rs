//! A Rust board support package (BSP) for the Teensy 4.
//!
//! The BSP is mainly a pass-through of the `imxrt-hal` hardware abstraction layer.
//! The BSP restricts the processor pads that are available, since the physical Teensy
//! only has a few user-accessible pins. From these pins, you may construct peripherals
//! and perform I/O.
//!
//! The BSP also exposes a USB logging interface. See the [`usb`](usb/index.html) module
//! for more details.
//!
//! The BSP does assume some facilities of the processor, both which are required for the
//! USB stack. Each are controllable through feature-flags. Each feature is on by default.
//!
//! - it registers the `SysTick` exception handler, and configures
//!   SYSTICK for a 1ms interrupt. Enabled with the `"systick"` feature,
//!   which is on by default.
//! - it registers the `USB_OTG1` interrupt, and uses the USB1
//!   peripheral for logging. Enabled with the `"usb-logging"` feature,
//!   which is on by default. Depends on the `"systick"` feature.
//!
//! These peripherals and capabilities are not exported from the BSP.
//! If a user also registers a `SysTick` or `USB_OTG1` handler, it may
//! result in a duplicate definition error.
//!
//! ## Re-exports
//!
//! The BSP re-exports the following:
//!
//! - the `teensy4-rt` crate, as `rt`
//! - the `imxrt-hal` crate, as `hal`
//!
//! See the accompanying documentation of each crate for more
//! information.
//!
//! For simplicity, there may be other choice APIs from either crate that
//! are re-exported in the BSP namespace.
//!
//! Although it's not exported publicly, the BSP crate links in the
//! `teensy4-fcb` crate, which provides a Firmware Configuration Block (FCB)
//! specific to the Teensy 4. See the `teensy4-fcb` crate for details
//! on FCBs.
//!
//! ## Physical Pins to Pads and Alternative Functions
//!
//! The sparse table below describes the Teensy 4 pins, the pad ID, and some of the notable alternative functionalities
//! for each pin. We add entries to the table as we add capabilities to the underlying HAL crate. Contributions to complete
//! this table are welcome! If a pad's alternatives are not listed here, consult the iMXRT1060 reference manual.
//!
//! | Pin  | Pad ID   |  Alt0    |  Alt1        |  Alt2        |  Alt3     |  Alt4        |  Alt5            |  Alt6        |  Alt7   |  Alt8   |  Alt9   |
//! | ---- | -------- | -------- | ------------ | ------------ | --------- | ------------ | ---------------- | ------------ | ------- | ------- | ------- |
//! |  0   |`AD_B0_03`|          |              |  `UART6_RX`  |           |`FlexPWM1_1_X`|                  |              |         |         |         |
//! |  1   |`AD_B0_02`|          |              |  `UART6_TX`  |           |`FlexPWM1_0_X`|                  |              |         |         |         |
//! |  2   |`EMC_04`  |          |`FlexPWM4_2_A`|              |           |              |                  |              |         |         |         |
//! |  3   |`EMC_05`  |          |`FlexPWM4_2_B`|              |           |              |                  |              |         |         |         |
//! |  4   |`EMC_06`  |          |`FlexPWM2_0_A`|              |           |              |                  |              |         |         |         |
//! |  5   |`EMC_08`  |          |`FlexPWM2_1_A`|              |           |              |                  |              |         |         |         |
//! |  6   |`B0_10`   |          |              |`FlexPWM2_2_A`|           |              |                  |              |         |         |         |
//! |  7   |`B1_01`   |          |              |  `UART4_RX`  |           |              |                  |`FlexPWM1_3_B`|         |         |         |
//! |  8   |`B1_00`   |          |              |  `UART4_TX`  |           |              |                  |`FlexPWM1_3_A`|         |         |         |
//! |  9   |`B0_11`   |          |              |`FlexPWM2_2_B`|           |              |                  |              |         |         |         |
//! |  10  |`B0_00`   |          |              |              |`SPI4_PCS0`|              |                  |              |         |         |         |
//! |  11  |`B0_02`   |          |              |              |`SPI4_SDO` |              |                  |              |         |         |         |
//! |  12  |`B0_01`   |          |              |              |`SPI4_SDI` |              |                  |              |         |         |         |
//! |  13  |`B0_03`   |          |              |              |`SPI4_SCK` |              |`GPIO2_3` (`LED`) |              |         |         |         |
//! |  14  |`AD_B1_02`|          |              |  `UART2_TX`  |           |              |                  |              |         |         |         |
//! |  15  |`AD_B1_03`|          |              |  `UART2_RX`  |           |              |                  |              |         |         |         |
//! |  16  |`AD_B1_07`|          |`I2C3_SCL`    |  `UART3_RX`  |           |              |                  |              |         |         |         |
//! |  17  |`AD_B1_06`|          |`I2C3_SDA`    |  `UART3_TX`  |           |              |                  |              |         |         |         |
//! |  18  |`AD_B1_01`|          |              |              |`I2C1_SDA` |              |                  |              |         |         |         |
//! |  19  |`AD_B1_00`|          |              |  `UART2_CTS` |`I2C1_SCL` |              |                  |              |         |         |         |
//! |  20  |`AD_B1_10`|          |              |  `UART8_TX`  |           |              |                  |              |         |         |         |
//! |  21  |`AD_B1_11`|          |              |  `UART8_RX`  |           |              |                  |              |         |         |         |
//! |  22  |`AD_B1_08`|          |`FlexPWM4_0_A`|              |           |              |                  |              |         |         |         |
//! |  23  |`AD_B1_09`|          |`FlexPWM4_1_A`|              |           |              |                  |              |         |         |         |
//! |  24  |`AD_B0_12`|`I2C4_SCL`|              |  `UART1_TX`  |           |`FlexPWM1_2_X`|                  |              |         |         |         |
//! |  25  |`AD_B0_13`|`I2C4_SDA`|              |  `UART1_RX`  |           |`FlexPWM1_3_X`|                  |              |         |         |         |
//! |  26  |`AD_B1_14`|          |              |              |           |              |                  |              |         |         |         |
//! |  27  |`AD_B1_15`|          |              |              |           |              |                  |              |         |         |         |
//! |  28  |`EMC_32`  |          |`FlexPWM3_1_B`|  `UART7_RX`  |           |              |                  |              |         |         |         |
//! |  29  |`EMC_31`  |          |`FlexPWM3_1_A`|  `UART7_TX`  |`SPI1_PCS1`|              |                  |              |         |         |         |
//! |  30  |`EMC_37`  |          |              |              |           |              |                  |              |         |         |         |
//! |  31  |`EMC_36`  |          |              |              |           |              |                  |              |         |         |         |
//! |  32  |`B0_12`   |          |              |              |           |              |                  |              |         |         |         |
//! |  33  |`EMC_07`  |          |`FlexPWM2_0_B`|              |           |              |                  |              |         |         |         |
//! |  34  |`SD_B0_03`|          |`FlexPWM1_1_B`|              |           |  `SPI1_SDI`  |                  |              |         |         |         |
//! |  35  |`SD_B0_02`|          |`FlexPWM1_1_A`|              |           |  `SPI1_SDO`  |                  |              |         |         |         |
//! |  36  |`SD_B0_01`|          |`FlexPWM1_0_B`|  `I2C3_SDA`  |           |  `SPI1_PCS0` |                  |              |         |         |         |
//! |  37  |`SD_B0_00`|          |`FlexPWM1_0_A`|  `I2C3_SCL`  |           |  `SPI1_SCK`  |                  |              |         |         |         |
//! |  38  |`SD_B0_05`|          |`FlexPWM1_2_B`|  `UART8_RX`  |           |              |                  |              |         |         |         |
//! |  39  |`SD_B0_04`|          |`FlexPWM1_2_A`|  `UART8_TX`  |           |              |                  |              |         |         |         |
//!
//! References:
//! - [Teensy 4.0 Schematic Diagram](https://www.pjrc.com/teensy/schematic.html)
//!
//! ## Examples
//!
//! See the `teensy4-examples` crate for build-able, run-able
//! examples. The examples utilize this BSP crate to blink LEDs,
//! establish timers, and log data over USB.
//!
//! ## Notice of alpha status
//!
//! We've made some assumptions in this MVP BSP.
//!
//! - SYSTICK and delay implementation is very naive. Do not run for 49
//!   continuous days, or risk a millisecond counter wrap-around.

#![no_std]

// Need to reference this so that it doesn't get stripped out
extern crate teensy4_fcb;

#[cfg(feature = "systick")]
mod systick;
#[cfg(feature = "usb-logging")]
pub mod usb;

#[cfg(feature = "systick")]
pub use systick::SysTick;

pub use hal::ral::interrupt;
// `rtic` expects these in the root.
#[cfg(feature = "rtic")]
pub use hal::ral::{interrupt as Interrupt, NVIC_PRIO_BITS};

pub use cortex_m_rt as rt;
pub use imxrt_hal as hal;

/// The LED in its final configuration
pub type LED = hal::gpio::GPIO<iomuxc::b0::B0_03, hal::gpio::Output>;

use hal::iomuxc;
use iomuxc::consts::{U1, U2, U3, U4};

/// Teensy pins that do not yet have a function
///
/// Pin 13 can be used for several things; one common usage is for the on-board LED.
pub struct Pins {
    /// Pin 0
    pub p0: iomuxc::ad_b0::AD_B0_03,
    /// Pin 1
    pub p1: iomuxc::ad_b0::AD_B0_02,
    /// Pin 2
    pub p2: iomuxc::emc::EMC_04,
    /// Pin 3
    pub p3: iomuxc::emc::EMC_05,
    /// Pin 4
    pub p4: iomuxc::emc::EMC_06,
    /// Pin 5
    pub p5: iomuxc::emc::EMC_08,
    /// Pin 6
    pub p6: iomuxc::b0::B0_10,
    /// Pin 7
    pub p7: iomuxc::b1::B1_01,
    /// Pin 8
    pub p8: iomuxc::b1::B1_00,
    /// Pin 9
    pub p9: iomuxc::b0::B0_11,
    /// Pin 10
    pub p10: iomuxc::b0::B0_00,
    /// Pin 11
    pub p11: iomuxc::b0::B0_02,
    /// Pin 12
    pub p12: iomuxc::b0::B0_01,
    /// Pin 13
    pub p13: iomuxc::b0::B0_03,
    /// Pin 14
    pub p14: iomuxc::ad_b1::AD_B1_02,
    /// Pin 15
    pub p15: iomuxc::ad_b1::AD_B1_03,
    /// Pin 16
    pub p16: iomuxc::ad_b1::AD_B1_07,
    /// Pin 17
    pub p17: iomuxc::ad_b1::AD_B1_06,
    /// Pin 18
    pub p18: iomuxc::ad_b1::AD_B1_01,
    /// Pin 19
    pub p19: iomuxc::ad_b1::AD_B1_00,
    /// Pin 20
    pub p20: iomuxc::ad_b1::AD_B1_10,
    /// Pin 21
    pub p21: iomuxc::ad_b1::AD_B1_11,
    /// Pin 22
    pub p22: iomuxc::ad_b1::AD_B1_08,
    /// Pin 23
    pub p23: iomuxc::ad_b1::AD_B1_09,
    /// Pin 24
    pub p24: iomuxc::ad_b0::AD_B0_12,
    /// Pin 25
    pub p25: iomuxc::ad_b0::AD_B0_13,
    /// Pin 28
    pub p28: iomuxc::emc::EMC_32,
    /// Pin 29
    pub p29: iomuxc::emc::EMC_31,
    /// Pin 33
    pub p33: iomuxc::emc::EMC_07,
    /// Pin 36
    pub p36: iomuxc::sd_b0::SD_B0_01,
    /// Pin 37
    pub p37: iomuxc::sd_b0::SD_B0_00,
}

/// All peripherals available on the Teensy4
///
/// Nearly all of these are re-exports from the HAL. Exclusions include
///
/// - `usb`, which is a USB logger
/// - `pins`, which are the Teensy 4's available pins
///
/// See the [module-level documentation](index.html) for more information.
#[non_exhaustive]
pub struct Peripherals {
    /// Clock control module (forwarded from the HAL)
    pub ccm: hal::ccm::CCM,
    /// PIT timers (forwarded from the HAL)
    pub pit: hal::pit::UnclockedPIT,
    /// The USB logger and serial reader
    #[cfg(feature = "usb-logging")]
    pub usb: usb::USB,
    /// DCDC converters
    pub dcdc: hal::dcdc::DCDC,
    /// PWM1 controller
    pub pwm1: hal::pwm::Unclocked<U1>,
    /// PWM2 controller
    pub pwm2: hal::pwm::Unclocked<U2>,
    /// PWM3 controller
    pub pwm3: hal::pwm::Unclocked<U3>,
    /// PWM4 controller
    pub pwm4: hal::pwm::Unclocked<U4>,
    /// Teensy pins
    pub pins: Pins,
    /// Unclocked I2C peripherals
    pub i2c: hal::i2c::Unclocked,
    /// Unclocked SPI peripherals
    pub spi: hal::spi::Unclocked,
    /// Unclocked UART peripherals
    pub uart: hal::uart::Unclocked,
    /// General purpose timer 1
    pub gpt1: hal::gpt::Unclocked,
    /// General purpose timer 2
    pub gpt2: hal::gpt::Unclocked,
    /// DMA channels
    pub dma: hal::dma::Unclocked,
    /// The SysTick delay timer
    #[cfg(feature = "systick")]
    pub systick: SysTick,
}

/// SYSTICK external clock frequency
///
/// See Section 12.3.2.1 of the reference manual. The note
/// explains that the 24MHz clock is divided down to 100KHz
/// before reaching SYSTICK.
const SYSTICK_EXT_FREQ: u32 = 100_000;

impl Peripherals {
    /// Instantiate the system peripherals. This may only be called once!
    pub fn take() -> Option<Self> {
        let p = hal::Peripherals::take()?;
        let mut cp = cortex_m::Peripherals::take()?;
        Self::set_systick(&mut cp.SYST);
        Some(Peripherals::new(p))
    }

    #[cfg(feature = "rtic")]
    /// Steal all of the HAL's peripherals.
    ///
    /// # Safety
    ///
    /// NOTE: This constructor is only intended for use with the `rtic` crate. This is **not** an
    /// alternative to the `take` constructor. The `take` constructor sets the system timer
    /// interrupt while this constructor does not seeing as `rtic` will take care of this for us.
    pub unsafe fn steal() -> Self {
        Self::new(hal::Peripherals::steal())
    }

    fn set_systick(systick: &mut cortex_m::peripheral::SYST) {
        systick.disable_counter();
        systick.set_clock_source(cortex_m::peripheral::syst::SystClkSource::External);
        systick.set_reload((SYSTICK_EXT_FREQ / 1000) - 1);
        systick.clear_current();
        systick.enable_counter();
        systick.enable_interrupt();
    }

    fn new(p: hal::Peripherals) -> Peripherals {
        Peripherals {
            ccm: p.ccm,
            pit: p.pit,
            #[cfg(feature = "usb-logging")]
            usb: usb::USB::new(),
            dcdc: p.dcdc,
            pwm1: p.pwm1,
            pwm2: p.pwm2,
            pwm3: p.pwm3,
            pwm4: p.pwm4,
            pins: Pins {
                p0: p.iomuxc.ad_b0.p03,
                p1: p.iomuxc.ad_b0.p02,
                p2: p.iomuxc.emc.p04,
                p3: p.iomuxc.emc.p05,
                p4: p.iomuxc.emc.p06,
                p5: p.iomuxc.emc.p08,
                p6: p.iomuxc.b0.p10,
                p7: p.iomuxc.b1.p01,
                p8: p.iomuxc.b1.p00,
                p9: p.iomuxc.b0.p11,
                p10: p.iomuxc.b0.p00,
                p11: p.iomuxc.b0.p02,
                p12: p.iomuxc.b0.p01,
                p13: p.iomuxc.b0.p03,
                p14: p.iomuxc.ad_b1.p02,
                p15: p.iomuxc.ad_b1.p03,
                p16: p.iomuxc.ad_b1.p07,
                p17: p.iomuxc.ad_b1.p06,
                p18: p.iomuxc.ad_b1.p01,
                p19: p.iomuxc.ad_b1.p00,
                p20: p.iomuxc.ad_b1.p10,
                p21: p.iomuxc.ad_b1.p11,
                p22: p.iomuxc.ad_b1.p08,
                p23: p.iomuxc.ad_b1.p09,
                p24: p.iomuxc.ad_b0.p12,
                p25: p.iomuxc.ad_b0.p13,
                p28: p.iomuxc.emc.p32,
                p29: p.iomuxc.emc.p31,
                p33: p.iomuxc.emc.p07,
                p36: p.iomuxc.sd_b0.p01,
                p37: p.iomuxc.sd_b0.p00,
            },
            i2c: p.i2c,
            spi: p.spi,
            uart: p.uart,
            gpt1: p.gpt1,
            gpt2: p.gpt2,
            dma: p.dma,
            #[cfg(feature = "systick")]
            systick: SysTick::new(),
        }
    }
}

/// Configure the board's LED
///
/// Returns a GPIO that's physically tied to the LED. Use the returned handle
/// to drive the LED.
pub fn configure_led(pad: iomuxc::b0::B0_03) -> LED {
    let mut led = hal::gpio::GPIO::new(pad);
    led.set_fast(true);
    led.output()
}

/// TODO(mciantyre) define a better yield
#[no_mangle]
fn r#yield() {
    // 'yield' is a Rust keyword. But, it needs to be called 'yield' for the C USB stack
    cortex_m::asm::delay(1024);
}

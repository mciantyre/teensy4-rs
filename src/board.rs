//! Pre-configured board resources.
//!
//! The `board` module conveniently exposes drivers and hardware resources.
//! It configures peripheral clocks and power so that you can simply use the
//! board's peripherals.
//!
//! # Getting started
//!
//! Create board resources depending on your board:
//!
//! - Teensy 4.0 boards should use [`t40`].
//! - Teensy 4.1 boards should use [`t41`].
//! - Teensy MicroMod boards should use [`tmm`].
//!
//! The produced resources are nearly the same, except for the pins.
//!
//! ```no_run
//! use teensy4_bsp as bsp;
//! use bsp::board;
//!
//! let resources = board::t40(board::instances());
//! ```
//!
//! Resources take ownership of all peripheral instances `Instances`. Use [`instances()`] to
//! acquire these once.
//!
//! > Note that if you're using RTIC, you can let the RTIC framework allocate
//! > the instances. The examples included in this package demonstrate this pattern.
//!
//! # Resource categories
//!
//! The objects in [`Resources`] have two main categories:
//!
//! 1. ready-to-go drivers, like `gpt1` and `dma`, which are unlikely to change no matter your
//!    hardware setup.
//! 2. lower-level resources, like `pins` and `lpuart3`, which you can compose to create drivers
//!    specific for your hardware setup.
//!
//! Given the Teensy 4's I/O flexibility, it would be difficult to expose all features as ready-to-go
//! drivers. Instead, `board` tries to help you initialize drivers for your hardware
//! setup. For example,
//!
//! - use [`led`] to prepare the on-board LED.
//! - use [`lpspi`] to prepare SPI peripherals.
//! - use [`lpuart`] to prepare serial peripherals.
//! - use [`lpi2c`] to prepare I2C peripherals.
//!
//! For the complete list of helper functions, study the rest of this API documentation and their
//! examples. And for the complete driver documentation, see [the `hal` module](crate::hal) and
//! its submodules.
//!
//! These helper functions assume that you're following the `board`'s clock (and power) policy.
//!
//! # Clock policy
//!
//! Once [`t40`] and [`t41`] return, the clocks of all returned resources are running, and clock gates are
//! enabled. Essentially, `board` handles CCM configurations so that you can use peripherals with reasonable
//! settings. `board` exposes constants describing these clock frequencies.
//!
//! Clock frequency constants, and all APIs depending on those constants, assume that you do not change the clock policy.
//! But for those who want extra control, `Resources` exposes the clock management peripherals. If you take this
//! route, you're responsible for ensuring your peripheral's clock configurations, and you should ignore the constants
//! exposed by this module.
//!
//! ## Clock details
//!
//! `board` makes a few guarantees about the clock policy. This section describes those guarantees.
//!
//! It's considered an API-breaking change to vary the frequencies and derivations of these clocks:
//!
//! - The ARM core frequency is documented in [`ARM_FREQUENCY`].
//! - PERCLK, the root clock for GPT and PIT timers, is documented in [`PERCLK_FREQUENCY`].
//! - The crystal oscillator is documented in [`XTAL_OSCILLATOR_FREQUENCY`].
//!
//! `board` does not touch the following root clock components. You should configure these components yourself.
//!
//! - FlexIO1 and FlexIO2 dividers and multiplexers.
//!
//! Other clock frequencies are exempt from this policy; they may change value or derivation without
//! notice. Nevertheless, if these clock frequencies / derivations change, `board` still guarantees that
//! the functions to set peripheral baud and clock rates will work as expected.
//!
//! By the time your code has access to [`Resources`], the clock gates for _all_ peripherals provided by
//! `Resources` is set to "on." If you're not using specific resources, you may configure their clock
//! gates to "off." `board` code that borrows or converts peripherals may assume that clock gates are
//! set to "on."
//!
//! If you're not using the BSP to prepare board `Resources`, consider using [`prepare_clocks_and_power`]
//! to realize the BSP's clock policy.
//!
//! # Naming conventions
//!
//! This module identifies peripherals by their hardware names. It does not use identifiers
//! from the Teensy 4 SDK and documentation. For instance, to name the serial peripheral
//! that uses pins 14 and 15,
//!
//! - this module uses the name "LPUART**2**".
//! - the Teensy 4 SDK / documentation uses the name "UART**3**".
//!
//! There's a few reasons for these differences:
//!
//! - It would be unintuitive to register a LPUART**2** *interrupt* for the peripheral named
//!   UART**3**. Since the BSP typically does not manage interrupt registration, this naming
//!   decision makes it easier for you to register and implement interrupts.
//! - It makes clear the peripheral implementation. You know that the serial peripheral
//!   supports low-power (LP) operation, and that it's not a FlexIO peripheral emulating a UART
//!   device.

use crate::{hal, pins, ral};
use core::sync::atomic::{AtomicBool, Ordering};
pub use hal::lpi2c::Lpi2c;
pub use hal::lpspi::Lpspi;
pub use hal::lpspi::Pins as LpspiPins;
pub use hal::lpuart::Lpuart;

/// Use [`instances()`] to safely acquire.
pub use ral::Instances;

/// Acquire peripheral instances.
///
/// These are the resources supplied to [a resource constructor](crate::board#getting-started).
/// They can also be used to initialize your own drivers.
///
/// ```
/// use teensy4_bsp as bsp;
/// use bsp::board;
///
/// let instances = board::instances();
/// ```
///
/// # Panics
///
/// Panics if the instances have already been taken from this function.
/// If you're using RTIC, know that RTIC may take these instances before
/// calling your `init` function.
///
/// ```should_panic
/// # use teensy4_bsp as bsp;
/// # use bsp::board;
/// let instances = board::instances();
/// // Sometime later...
/// board::instances(); // Panics! Instances already taken.
/// ```
///
/// # Safety
///
/// This function provides a safe, convenient way to acquire one [`Instances`] object.
/// It's considered safe for the use-cases that `teensy4-bsp` intends to support;
/// specifically, you're building one binary that represents the entire firmware
/// image, and that there's no other software which is acquiring these resources.
pub fn instances() -> Instances {
    static TAKEN: AtomicBool = AtomicBool::new(false);
    assert!(!TAKEN.swap(true, Ordering::SeqCst));
    unsafe { Instances::instances() }
}

pub use crate::clock_power::*;

/// Resources for a Teensy 4.0.
///
/// Use [`t40`] to construct this. The pins are specific to the Teensy 4.0.
pub type T40Resources = Resources<pins::t40::Pins>;
/// Resources for a Teensy 4.1.
///
/// Use [`t41`] to construct this. The pins are specific to the Teensy 4.1.
pub type T41Resources = Resources<pins::t41::Pins>;
/// Resources for a Teensy MicroMod.
///
/// Use [`tmm`] to construct this. The pins are specific to the Teensy MicroMod.
pub type TMMResources = Resources<pins::tmm::Pins>;

/// Resources constructed by the board.
///
/// The concrete `Pins` type depends on how this is constructed.
/// See the various `*Resources` aliases for more information.
#[non_exhaustive]
pub struct Resources<Pins> {
    /// Periodic interrupt timer.
    pub pit: hal::pit::Pit,
    /// General purpose timer 1.
    pub gpt1: hal::gpt::Gpt,
    /// General purpose timer 2.
    pub gpt2: hal::gpt::Gpt,
    /// GPIO1 port.
    pub gpio1: hal::gpio::Port,
    /// GPIO2 port.
    pub gpio2: hal::gpio::Port,
    /// GPIO3 port.
    pub gpio3: hal::gpio::Port,
    /// GPIO4 port.
    pub gpio4: hal::gpio::Port,
    /// USB1 instances.
    ///
    /// Use this to construct higher-level USB drivers, or to initialize the USB logger.
    pub usb: crate::usbd::Instances<1>,
    /// DMA channels.
    pub dma: [Option<hal::dma::channel::Channel>; hal::dma::CHANNEL_COUNT],
    /// The secure real-time counter.
    ///
    /// It's initially disabled, and you may enable it in your firmware.
    pub srtc: hal::snvs::srtc::Disabled,
    /// Core registers for the SNVS low-power domain.
    ///
    /// Use this with the SRTC and other SNVS LP components.
    pub snvs_lp_core: hal::snvs::LpCore,
    /// Clock control module.
    pub ccm: ral::ccm::CCM,
    /// Analog clock control module.
    pub ccm_analog: ral::ccm_analog::CCM_ANALOG,
    /// DCDC converter
    pub dcdc: ral::dcdc::DCDC,
    /// All available pins.
    pub pins: Pins,
    /// The register block for LPI2C1.
    pub lpi2c1: ral::lpi2c::LPI2C1,
    /// The register block for LPI2C3.
    pub lpi2c3: ral::lpi2c::LPI2C3,
    /// The register block for LPSPI1.
    pub lpspi1: ral::lpspi::LPSPI1,
    /// The register block for LPSPI2.
    pub lpspi2: ral::lpspi::LPSPI2,
    /// The register block for LPSPI3.
    pub lpspi3: ral::lpspi::LPSPI3,
    /// The register block for LPSPI4.
    pub lpspi4: ral::lpspi::LPSPI4,
    /// The register block for LPUART6.
    pub lpuart6: ral::lpuart::LPUART6,
    /// The register block for LPUART4.
    pub lpuart4: ral::lpuart::LPUART4,
    /// The register block for LPUART2.
    pub lpuart2: ral::lpuart::LPUART2,
    /// The register block for LPUART3.
    pub lpuart3: ral::lpuart::LPUART3,
    /// The register block for LPUART8.
    pub lpuart8: ral::lpuart::LPUART8,
    /// The register block for LPUART1.
    pub lpuart1: ral::lpuart::LPUART1,
    /// The register block for LPUART5.
    pub lpuart5: ral::lpuart::LPUART5,
    /// The register block for LPUART7.
    pub lpuart7: ral::lpuart::LPUART7,
    /// FlexPWM1.
    pub flexpwm1: hal::flexpwm::Pwm,
    /// FlexPWM2.
    pub flexpwm2: hal::flexpwm::Pwm,
    /// FlexPWM3.
    pub flexpwm3: hal::flexpwm::Pwm,
    /// FlexPWM4.
    pub flexpwm4: hal::flexpwm::Pwm,
    /// The FlexIO1 register block.
    pub flexio1: ral::flexio::FLEXIO1,
    /// The FlexIO2 register block.
    pub flexio2: ral::flexio::FLEXIO2,
    /// The FlexIO3 register block.
    pub flexio3: ral::flexio::FLEXIO3,
    /// The register block for ADC1.
    ///
    /// ADC drivers constructed by `board` use a pre-configured clock and divisor. To change
    /// this configuration, call `release()` to acquire the register block, then re-construct
    /// the driver.
    pub adc1: hal::adc::Adc,
    /// The register block for ADC2.
    pub adc2: hal::adc::Adc,
    /// True random number generator.
    pub trng: hal::trng::Trng,
    /// Temperature monitor of the core.
    pub tempmon: hal::tempmon::TempMon,
    /// The register block for SAI1 (I2S audio).
    ///
    /// SAI1 is the primary audio interface used by the Teensy Audio Shield.
    pub sai1: ral::sai::SAI1,
    /// The register block for SAI2.
    pub sai2: ral::sai::SAI2,
    /// The register block for SAI3.
    pub sai3: ral::sai::SAI3,
    /// The IOMUXC general purpose register block.
    pub iomuxc_gpr: ral::iomuxc_gpr::IOMUXC_GPR,
    /// The USDHC1 peripheral instance.
    pub usdhc1: ral::usdhc::USDHC1,
    /// The FlexSPI2 peripheral instance.
    pub flexspi2: ral::flexspi::FLEXSPI2,
}

/// The board's dedicated LED.
pub type Led = hal::gpio::Output;

/// Create the board's LED.
///
/// ```no_run
/// use teensy4_bsp as bsp;
/// use bsp::board;
///
/// let board::Resources { mut gpio2, pins, .. }
///     = board::t40(board::instances());
///
/// let led = board::led(&mut gpio2, pins.p13);
/// ```
pub fn led(gpio2: &mut hal::gpio::Port, p13: pins::common::P13) -> Led {
    gpio2.output(p13).expect("P13 is a GPIO2 pin")
}

/// Create a LPI2C peripheral.
///
/// Consider using explicit type annotations, as demonstrated below,
/// to simplify any errors from the compiler.
///
/// This helper assumes that the actual LPI2C clock frequency equals [`LPI2C_FREQUENCY`].
///
/// ```no_run
/// use teensy4_bsp as bsp;
/// use bsp::board;
///
/// let board::Resources { lpi2c3, pins, ..}
///     = board::t40(board::instances());
///
/// let mut lpi2c: board::Lpi2c = board::lpi2c(
///     lpi2c3,
///     pins.p16,
///     pins.p17,
///     board::Lpi2cClockSpeed::KHz400,
/// );
/// ```
pub fn lpi2c<Scl, Sda, const N: u8>(
    instance: ral::lpi2c::Instance<N>,
    scl: Scl,
    sda: Sda,
    clock_speed: Lpi2cClockSpeed,
) -> hal::lpi2c::Lpi2c
where
    Scl: hal::iomuxc::lpi2c::Pin<
            Signal = hal::iomuxc::lpi2c::Scl,
            Module = hal::iomuxc::consts::Const<N>,
        >,
    Sda: hal::iomuxc::lpi2c::Pin<
            Signal = hal::iomuxc::lpi2c::Sda,
            Module = hal::iomuxc::consts::Const<N>,
        >,
{
    hal::lpi2c::Lpi2c::with_pins(
        instance,
        hal::lpi2c::Pins { scl, sda },
        &lpi2c_baud(clock_speed),
    )
}

/// Create a LPSPI peripheral.
///
/// `baud` is the SCK frequency, in Hz. Consider using explicit type annotations, as demonstrated
/// below, to simplify any errors from the compiler.
///
/// This helper assumes that the LPSPI clock frequency equals [`LPSPI_FREQUENCY`].
///
/// ```no_run
/// use teensy4_bsp as bsp;
/// use bsp::board;
///
/// let board::T40Resources { lpspi4, pins, .. }
///     = board::t40(board::instances());
///
/// let mut lpspi4: board::Lpspi = board::lpspi(
///     lpspi4,
///     board::LpspiPins {
///         sdo: pins.p11,
///         sdi: pins.p12,
///         sck: pins.p13,
///     },
///     1_000_000,
/// );
/// ```
pub fn lpspi<Sdo, Sdi, Sck, const N: u8>(
    instance: ral::lpspi::Instance<N>,
    pins: LpspiPins<Sdo, Sdi, Sck>,
    baud: u32,
) -> hal::lpspi::Lpspi
where
    Sdo: hal::iomuxc::lpspi::Pin<
            Signal = hal::iomuxc::lpspi::Sdo,
            Module = hal::iomuxc::consts::Const<N>,
        >,
    Sdi: hal::iomuxc::lpspi::Pin<
            Signal = hal::iomuxc::lpspi::Sdi,
            Module = hal::iomuxc::consts::Const<N>,
        >,
    Sck: hal::iomuxc::lpspi::Pin<
            Signal = hal::iomuxc::lpspi::Sck,
            Module = hal::iomuxc::consts::Const<N>,
        >,
{
    let mut spi = hal::lpspi::Lpspi::with_pins(instance, pins);
    spi.disabled(|spi| spi.set_clock_hz(LPSPI_FREQUENCY, baud));
    spi
}

/// Create a LPUART peripheral.
///
/// The specific peripheral you're creating depends on
///
/// - the type of `instance`.
/// - the TX and RX pins.
/// - the return type, which can be explicitly annotated.
///
/// Consider using an explicit type annotation to help catch any
/// programming errors.
///
/// This helper assumes that the UART clock frequency equals [`UART_FREQUENCY`].
///
/// ```no_run
/// use teensy4_bsp as bsp;
/// use bsp::board;
///
/// let board::T40Resources { lpuart6, lpuart2, pins, .. }
///     = board::t40(board::instances());
///
/// // Explicit type:
/// let mut lpuart6: board::Lpuart = board::lpuart(
///     lpuart6,
///     pins.p1,
///     pins.p0,
///     115200,
/// );
///
/// // Implicit type:
/// let mut lpuart2 = board::lpuart(lpuart2, pins.p14, pins.p15, 9600);
/// ```
pub fn lpuart<Tx, Rx, const N: u8>(
    instance: ral::lpuart::Instance<N>,
    tx: Tx,
    rx: Rx,
    baud: u32,
) -> hal::lpuart::Lpuart
where
    Tx: hal::iomuxc::lpuart::Pin<
            Direction = hal::iomuxc::lpuart::Tx,
            Module = hal::iomuxc::consts::Const<N>,
        >,
    Rx: hal::iomuxc::lpuart::Pin<
            Direction = hal::iomuxc::lpuart::Rx,
            Module = hal::iomuxc::consts::Const<N>,
        >,
{
    let mut uart = hal::lpuart::Lpuart::with_pins(instance, hal::lpuart::Pins { tx, rx });
    uart.disable(|uart| uart.set_baud(&lpuart_baud(baud)));
    uart
}

fn prepare_resources<Pins>(
    mut instances: Instances,
    from_pads: impl FnOnce(hal::iomuxc::pads::Pads) -> Pins,
) -> Resources<Pins> {
    prepare_clocks_and_power(
        &mut instances.CCM,
        &mut instances.CCM_ANALOG,
        &mut instances.DCDC,
    );
    let iomuxc = hal::iomuxc::into_pads(instances.IOMUXC);
    let pins = from_pads(iomuxc);

    // Stop timers in debug mode.
    ral::modify_reg!(ral::pit, instances.PIT, MCR, FRZ: FRZ_1);
    let pit = hal::pit::Pit::new(instances.PIT);

    let mut gpt1 = hal::gpt::Gpt::new(instances.GPT1);
    gpt1.disable();

    let mut gpt2 = hal::gpt::Gpt::new(instances.GPT2);
    gpt2.disable();

    let hal::snvs::Snvs {
        low_power:
            hal::snvs::LowPower {
                core: snvs_lp_core,
                srtc,
                ..
            },
        ..
    } = hal::snvs::new(instances.SNVS);

    let adc1 = hal::adc::Adc::new(
        instances.ADC1,
        hal::adc::ClockSelect::ADACK,
        hal::adc::ClockDivision::Div8,
    );
    let adc2 = hal::adc::Adc::new(
        instances.ADC2,
        hal::adc::ClockSelect::ADACK,
        hal::adc::ClockDivision::Div8,
    );

    let trng = hal::trng::Trng::new(instances.TRNG, Default::default(), Default::default());
    let tempmon = hal::tempmon::TempMon::with_measure_freq(instances.TEMPMON, 0x1000);

    Resources {
        pit,
        gpt1,
        gpt2,
        gpio1: hal::gpio::Port::new(instances.GPIO1),
        gpio2: hal::gpio::Port::new(instances.GPIO2),
        gpio3: hal::gpio::Port::new(instances.GPIO3),
        gpio4: hal::gpio::Port::new(instances.GPIO4),
        usb: crate::usbd::Instances {
            usb: instances.USB1,
            usbphy: instances.USBPHY1,
            usbnc: instances.USBNC1,
        },
        dma: hal::dma::channels(instances.DMA, instances.DMAMUX),
        srtc,
        snvs_lp_core,
        ccm: instances.CCM,
        ccm_analog: instances.CCM_ANALOG,
        dcdc: instances.DCDC,
        pins,
        lpi2c1: instances.LPI2C1,
        lpi2c3: instances.LPI2C3,
        lpspi1: instances.LPSPI1,
        lpspi2: instances.LPSPI2,
        lpspi3: instances.LPSPI3,
        lpspi4: instances.LPSPI4,
        lpuart6: instances.LPUART6,
        lpuart4: instances.LPUART4,
        lpuart2: instances.LPUART2,
        lpuart3: instances.LPUART3,
        lpuart8: instances.LPUART8,
        lpuart1: instances.LPUART1,
        lpuart7: instances.LPUART7,
        lpuart5: instances.LPUART5,
        flexio1: instances.FLEXIO1,
        flexio2: instances.FLEXIO2,
        flexio3: instances.FLEXIO3,
        flexpwm1: hal::flexpwm::Pwm::new(instances.PWM1),
        flexpwm2: hal::flexpwm::Pwm::new(instances.PWM2),
        flexpwm3: hal::flexpwm::Pwm::new(instances.PWM3),
        flexpwm4: hal::flexpwm::Pwm::new(instances.PWM4),
        adc1,
        adc2,
        trng,
        tempmon,
        sai1: instances.SAI1,
        sai2: instances.SAI2,
        sai3: instances.SAI3,
        iomuxc_gpr: instances.IOMUXC_GPR,
        usdhc1: instances.USDHC1,
        flexspi2: instances.FLEXSPI2,
    }
}

/// Create resources for the Teensy 4.0 board.
///
/// Note that the peripheral instances acquired by RTIC -- named `device` in the
/// `init::Context` object -- can be used as the argument to this function.
pub fn t40(instances: impl Into<Instances>) -> T40Resources {
    prepare_resources(instances.into(), pins::t40::from_pads)
}

/// Create resources for the Teensy 4.1 board.
///
/// Note that the peripheral instances acquired by RTIC -- named `device` in the
/// `init::Context` object -- can be used as the argument to this function.
pub fn t41(instances: impl Into<Instances>) -> T41Resources {
    prepare_resources(instances.into(), pins::t41::from_pads)
}

/// Create resources for the Teensy MicroMod board.
///
/// Note that the peripheral instances acquired by RTIC -- named `device` in the
/// `init::Context` object -- can be used as the argument to this function.
pub fn tmm(instances: impl Into<Instances>) -> TMMResources {
    prepare_resources(instances.into(), pins::tmm::from_pads)
}

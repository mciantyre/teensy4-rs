//! Implements the board's clock and power policy.

#![allow(clippy::assertions_on_constants)]

use crate::{
    hal::{
        self,
        ccm::{self, clock_gate, XTAL_OSCILLATOR_HZ},
        dcdc,
    },
    ral,
};

pub use hal::lpi2c::ClockSpeed as Lpi2cClockSpeed;

/// Frequency (Hz) of the crystal oscillator.
///
/// This is 24MHz.
pub const XTAL_OSCILLATOR_FREQUENCY: u32 = XTAL_OSCILLATOR_HZ;

const PLL1_DIV_SEL: u32 = 100;
const ARM_DIVIDER: u32 = 2;
const AHB_DIVIDER: u32 = 1;
const IPG_DIVIDER: u32 = 4;

/// Frequency (Hz) of the ARM core.
///
/// `board` guarantees that the ARM core runs at 600MHz. It's derived from
/// PLL1.
pub const ARM_FREQUENCY: u32 =
    ccm::analog::pll1::frequency(PLL1_DIV_SEL) / ARM_DIVIDER / AHB_DIVIDER;
const _: () = assert!(ARM_FREQUENCY == 600_000_000);

/// Frequency (Hz) of the IPG bus.
pub const IPG_FREQUENCY: u32 = ARM_FREQUENCY / IPG_DIVIDER;
const _: () = assert!(IPG_FREQUENCY == 150_000_000);

/// Prepares the AHB and IPG clock root.
fn setup_ahb_ipg_clk(ccm: &mut ral::ccm::CCM, ccm_analog: &mut ral::ccm_analog::CCM_ANALOG) {
    clock_gate::IPG_CLOCK_GATES
        .iter()
        .for_each(|locator| locator.set(ccm, clock_gate::OFF));

    if ccm::ahb_clk::Selection::PeriphClk2Sel == ccm::ahb_clk::selection(ccm) {
        // Switch to the pre-peripheral clock before changing
        // peripheral clock 2...
        ccm::ahb_clk::set_selection(ccm, ccm::ahb_clk::Selection::PrePeriphClkSel);
    }

    // Temporarily switch to the crystal oscillator.
    ccm::periph_clk2::set_divider(ccm, 1);
    ccm::periph_clk2::set_selection(ccm, ccm::periph_clk2::Selection::Osc);
    ccm::ahb_clk::set_selection(ccm, ccm::ahb_clk::Selection::PeriphClk2Sel);

    // Prepare PLL1.
    ccm::analog::pll1::restart(ccm_analog, PLL1_DIV_SEL);
    ccm::arm_divider::set_divider(ccm, ARM_DIVIDER);
    ccm::ahb_clk::set_divider(ccm, AHB_DIVIDER);

    // Switch back to PLL1.
    ccm::pre_periph_clk::set_selection(ccm, ccm::pre_periph_clk::Selection::Pll1);
    ccm::ahb_clk::set_selection(ccm, ccm::ahb_clk::Selection::PrePeriphClkSel);

    ccm::ipg_clk::set_divider(ccm, IPG_DIVIDER);
}

const PERCLK_DIVIDER: u32 = 24;
/// PERCLK clock frequency (Hz).
///
/// This is the PIT timer frequency. It can also be used as the GPT frequency,
/// provided you use the peripheral clock or high-frequency reference clock as
/// the GPT clock selection.
///
/// PERCLK runs at 1MHz. It derives from the crystal oscillator.
pub const PERCLK_FREQUENCY: u32 = XTAL_OSCILLATOR_FREQUENCY / PERCLK_DIVIDER;
const _: () = assert!(PERCLK_FREQUENCY == 1_000_000);

/// Prepare PERCLK for PIT and GPT timers.
fn setup_perclk_clk(ccm: &mut ral::ccm::CCM) {
    clock_gate::PERCLK_CLOCK_GATES
        .iter()
        .for_each(|locator| locator.set(ccm, clock_gate::OFF));
    ccm::perclk_clk::set_divider(ccm, PERCLK_DIVIDER);
    ccm::perclk_clk::set_selection(ccm, ccm::perclk_clk::Selection::Oscillator);
}

const UART_DIVIDER: u32 = 3;
/// Frequency (Hz) of all UARTs.
///
/// Use this to compute any alternate baud rates that aren't supported
/// by [`lpuart_baud()`].
pub const UART_FREQUENCY: u32 = XTAL_OSCILLATOR_FREQUENCY / UART_DIVIDER;

/// Prepare UART root clock for all LPUARTs.
fn setup_uart_clk(ccm: &mut ral::ccm::CCM) {
    clock_gate::UART_CLOCK_GATES
        .iter()
        .for_each(|locator| locator.set(ccm, clock_gate::OFF));

    ccm::uart_clk::set_selection(ccm, ccm::uart_clk::Selection::Oscillator);
    ccm::uart_clk::set_divider(ccm, UART_DIVIDER);
}

/// Type for LPUART baud rates.
///
/// Use [`lpuart_baud()`] to easily compute a baud rate.
pub type LpuartBaud = hal::lpuart::Baud;

/// Computes a UART baud rate.
///
/// Note that this can evaluate at compile time. This function assumes that
/// the UART clock matches [`UART_FREQUENCY`].
///
/// ```
/// use teensy4_bsp as bsp;
/// use teensy4_bsp::board;
///
/// const MY_BAUD: board::LpuartBaud = board::lpuart_baud(9600);
/// ```
pub const fn lpuart_baud(bps: u32) -> hal::lpuart::Baud {
    hal::lpuart::Baud::compute(UART_FREQUENCY, bps)
}

const LPI2C_DIVIDER: u32 = 3;
const LPI2C_CLOCK_SOURCE: ccm::lpi2c_clk::Selection = ccm::lpi2c_clk::Selection::Oscillator;

/// Frequency (Hz) of all LPI2C peripherals.
pub const LPI2C_FREQUENCY: u32 = XTAL_OSCILLATOR_FREQUENCY / LPI2C_DIVIDER;
const _: () = assert!(LPI2C_FREQUENCY == 8_000_000);

/// Prepare the LPI2C clock root.
fn setup_lpi2c_clk(ccm: &mut ral::ccm::CCM) {
    clock_gate::LPI2C_CLOCK_GATES
        .iter()
        .for_each(|locator| locator.set(ccm, clock_gate::OFF));
    ccm::lpi2c_clk::set_divider(ccm, LPI2C_DIVIDER);
    ccm::lpi2c_clk::set_selection(ccm, LPI2C_CLOCK_SOURCE);
}

/// LPI2C timing parameters.
pub type Lpi2cBaud = hal::lpi2c::Timing;

/// Computes a LPI2C baud rate, assuming ideal bus behavior.
///
/// If this produces timing configurations doesn't closely approximate your
/// expected baud rate, you may have more success by defining the LPI2C timing
/// configurations yourself.
///
/// This function assumes that the LPI2C clock frequency matches [`LPI2C_FREQUENCY`].
/// Note that this can evaluate at compile time.
///
/// ```no_run
/// use teensy4_bsp as bsp;
/// use bsp::board;
///
/// const MY_BAUD: board::Lpi2cBaud = board::lpi2c_baud(board::Lpi2cClockSpeed::KHz400);
/// ```
pub const fn lpi2c_baud(clock_speed: Lpi2cClockSpeed) -> Lpi2cBaud {
    Lpi2cBaud::ideal(LPI2C_FREQUENCY, clock_speed)
}

const LPSPI_DIVIDER: u32 = 4;
/// Frequency (Hz) of all LPSPI clocks.
pub const LPSPI_FREQUENCY: u32 = ccm::analog::pll2::FREQUENCY / LPSPI_DIVIDER;
const _: () = assert!(LPSPI_FREQUENCY == 132_000_000);

/// Prepare the LPSPI clock root.
fn setup_lpspi_clk(ccm: &mut ral::ccm::CCM) {
    clock_gate::LPSPI_CLOCK_GATES
        .iter()
        .for_each(|locator| locator.set(ccm, clock_gate::OFF));
    ccm::lpspi_clk::set_selection(ccm, ccm::lpspi_clk::Selection::Pll2);
    ccm::lpspi_clk::set_divider(ccm, LPSPI_DIVIDER);
}

const CLOCK_GATES: &[clock_gate::Locator] = &[
    clock_gate::pit(),
    clock_gate::gpt_bus::<1>(),
    clock_gate::gpt_bus::<2>(),
    clock_gate::gpt_serial::<1>(),
    clock_gate::gpt_serial::<2>(),
    clock_gate::gpio::<1>(),
    clock_gate::gpio::<2>(),
    clock_gate::gpio::<3>(),
    clock_gate::gpio::<4>(),
    clock_gate::usb(),
    clock_gate::dma(),
    clock_gate::snvs_lp(),
    clock_gate::snvs_hp(),
    clock_gate::lpi2c::<1>(),
    clock_gate::lpi2c::<3>(),
    clock_gate::lpspi::<1>(),
    clock_gate::lpspi::<2>(),
    clock_gate::lpspi::<3>(),
    clock_gate::lpspi::<4>(),
    clock_gate::lpuart::<6>(),
    clock_gate::lpuart::<4>(),
    clock_gate::lpuart::<2>(),
    clock_gate::lpuart::<3>(),
    clock_gate::lpuart::<8>(),
    clock_gate::lpuart::<1>(),
    clock_gate::lpuart::<7>(),
    clock_gate::lpuart::<5>(),
    clock_gate::flexpwm::<1>(),
    clock_gate::flexpwm::<2>(),
    clock_gate::flexpwm::<3>(),
    clock_gate::flexpwm::<4>(),
    clock_gate::flexio::<1>(),
    clock_gate::flexio::<2>(),
    clock_gate::flexio::<3>(),
    clock_gate::adc::<1>(),
    clock_gate::adc::<2>(),
    clock_gate::trng(),
];

/// Prepare clocks and power for the MCU.
///
/// This implements the [`board`](crate::board#clock-policy)'s clock policy. This
/// function is automatically called when acquiring board resources.
pub fn prepare_clocks_and_power(
    ccm: &mut ral::ccm::CCM,
    ccm_analog: &mut ral::ccm_analog::CCM_ANALOG,
    dcdc: &mut ral::dcdc::DCDC,
) {
    ccm::set_low_power_mode(ccm, ccm::LowPowerMode::RemainInRun);
    dcdc::set_target_vdd_soc(dcdc, 1250);
    ccm::analog::pll3::restart(ccm_analog);

    setup_ahb_ipg_clk(ccm, ccm_analog);
    setup_lpi2c_clk(ccm);
    setup_lpspi_clk(ccm);
    setup_perclk_clk(ccm);
    setup_uart_clk(ccm);

    CLOCK_GATES
        .iter()
        .for_each(|locator| locator.set(ccm, clock_gate::ON));
}

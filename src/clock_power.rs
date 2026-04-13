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

// --- Audio PLL (PLL4) and SAI clock configuration ---
//
// The Audio PLL is configured to produce a MCLK that yields a ~44117.647 Hz sample rate.
// This mirrors the Teensy Audio Library's C++ `set_audioClock(28, 2348, 10000)`:
//
//   PLL4 = 24 MHz × (28 + 2348/10000) / 1 = 677,635,200 Hz
//   SAI1_CLK = PLL4 / prediv(4) / podf(15) = 11,293,920 Hz (MCLK)
//   Sample rate = MCLK / 256 ≈ 44,117.656 Hz
//

/// Audio PLL (PLL4) divider selection.
const AUDIO_PLL_DIV_SELECT: u32 = 28;
/// Audio PLL numerator.
const AUDIO_PLL_NUM: u32 = 2348;
/// Audio PLL denominator.
const AUDIO_PLL_DENOM: u32 = 10000;

/// SAI clock predivider (1..=8).
const SAI_CLK_PREDIVIDER: u32 = 4;
/// SAI clock post-divider (1..=64).
const SAI_CLK_DIVIDER: u32 = 15;

/// Frequency (Hz) of the SAI1 MCLK after all dividers.
///
/// Approximately 11,293,920 Hz, yielding ~44,117.656 Hz sample rate
/// when MCLK/BCLK ratio is 256.
pub const SAI1_FREQUENCY: u32 = {
    // PLL4 output frequency (with post_div = 1):
    // 24_000_000 * (28 + 2348/10000) = 24_000_000 * 28 + 24_000_000 * 2348 / 10000
    let pll4_freq: u32 = XTAL_OSCILLATOR_HZ * AUDIO_PLL_DIV_SELECT
        + XTAL_OSCILLATOR_HZ / AUDIO_PLL_DENOM * AUDIO_PLL_NUM;
    pll4_freq / SAI_CLK_PREDIVIDER / SAI_CLK_DIVIDER
};

/// Configure the Audio PLL (PLL4) for 44.1 kHz–family sample rates.
fn setup_audio_pll(ccm_analog: &mut ral::ccm_analog::CCM_ANALOG) {
    ccm::analog::pll4::reconfigure(
        ccm_analog,
        AUDIO_PLL_DIV_SELECT,
        AUDIO_PLL_NUM,
        AUDIO_PLL_DENOM,
        ccm::analog::pll4::PostDivider::U1,
    );
}

/// Configure SAI1 clock root to derive from Audio PLL.
fn setup_sai1_clk(ccm: &mut ral::ccm::CCM) {
    clock_gate::sai::<1>().set(ccm, clock_gate::OFF);
    ccm::sai_clk::set_selection::<1>(ccm, ccm::sai_clk::Selection::Pll4);
    ccm::sai_clk::set_predivider::<1>(ccm, SAI_CLK_PREDIVIDER);
    ccm::sai_clk::set_divider::<1>(ccm, SAI_CLK_DIVIDER);
}

/// PLL2_PFD2 fractional divider. This value is set by the Teensy bootloader
/// and assumed to be stable before user code runs.
const PLL2_PFD2_FRAC: u32 = 24;
const PLL2_PFD2_FREQUENCY: u32 = ccm::analog::pll2::FREQUENCY / PLL2_PFD2_FRAC * 18;

const USDHC1_CLK_DIVISOR: u32 = 2;

/// USDHC1 root clock frequency (Hz).
///
/// Derived from PLL2_PFD2 (396 MHz) divided by `USDHC1_CLK_DIVISOR`. The actual
/// SD bus clock is further divided by the USDHC1 peripheral's internal
/// SDCLKFS and DVS dividers during card initialization.
pub const USDHC1_FREQUENCY: u32 = PLL2_PFD2_FREQUENCY / USDHC1_CLK_DIVISOR;
const _: () = assert!(USDHC1_FREQUENCY == 198_000_000);

/// Configure the USDHC1 clock root for the SD card slot.
///
/// Source: PLL2_PFD2 (396 MHz), divider: /2 → 198 MHz root clock.
///
/// PLL2 (528 MHz) and its PFD2 output (396 MHz) are initialized by the
/// Teensy bootloader before transferring control to user code. This
/// function assumes PLL2_PFD2 is already running and stable.
fn setup_usdhc1_clk(ccm: &mut ral::ccm::CCM) {
    clock_gate::usdhc::<1>().set(ccm, clock_gate::OFF);
    ral::modify_reg!(ral::ccm, ccm, CSCMR1, USDHC1_CLK_SEL: 0); // PLL2_PFD2
    ral::modify_reg!(ral::ccm, ccm, CSCDR1, USDHC1_PODF: USDHC1_CLK_DIVISOR - 1);
}

const FLEXSPI2_CLK_DIVISOR: u32 = 5;

/// FlexSPI2 serial clock frequency (Hz).
pub const FLEXSPI2_FREQUENCY: u32 = ccm::analog::pll2::FREQUENCY / FLEXSPI2_CLK_DIVISOR;
const _: () = assert!(FLEXSPI2_FREQUENCY == 105_600_000);

/// Configure the FlexSPI2 clock root for PSRAM.
///
/// Source: PLL2 (528 MHz), divider: /5 → 105.6 MHz serial clock.
fn setup_flexspi2_clk(ccm: &mut ral::ccm::CCM) {
    clock_gate::flexspi::<2>().set(ccm, clock_gate::OFF);
    ral::modify_reg!(ral::ccm, ccm, CBCMR,
        FLEXSPI2_PODF: FLEXSPI2_CLK_DIVISOR - 1,
        FLEXSPI2_CLK_SEL: FLEXSPI2_CLK_SEL_3   // PLL2 (528 MHz)
    );
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
    clock_gate::sai::<1>(),
    clock_gate::sai::<2>(),
    clock_gate::sai::<3>(),
    clock_gate::usdhc::<1>(),
    clock_gate::flexspi::<2>(),
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
    setup_audio_pll(ccm_analog);
    setup_sai1_clk(ccm);
    setup_usdhc1_clk(ccm);
    setup_flexspi2_clk(ccm);

    CLOCK_GATES
        .iter()
        .for_each(|locator| locator.set(ccm, clock_gate::ON));
}

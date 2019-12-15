//! Implements the routine to configure the processor speed
//!
//! Reimplementation of the [`set_arm_clock` routine] from the Teensy 4
//! Arduino Libraries.
//!
//! [`set_arm_clock` routine]: https://github.com/PaulStoffregen/cores/blob/master/teensy4/clockspeed.c

use imxrt1060_pac as pac;
use pac::{CCM, CCM_ANALOG, DCDC};

pub fn set_arm_clock(mut hz: u32, ccm: &CCM, ccm_analog: &CCM_ANALOG, dcdc: &DCDC) -> u32 {
    let millivolts: u32 = if hz > 528_000_000 {
        1250 // 1.25V
    } else if hz <= 24_000_000 {
        950 // 0.95V
    } else {
        1150 // 1.15V, default
    };

    // Enable clocks to the DCDC module
    // Safety: CG3 field is two bits
    ccm.ccgr6.modify(|_, w| unsafe { w.cg3().bits(0x3) });

    #[inline(always)]
    fn reg3_trg(mv: u32) -> u8 {
        ((mv - 800) / 25) as u8
    }

    // Set VDD_SOC, voltage for the chip
    if dcdc.reg3.read().trg().bits() < reg3_trg(millivolts) {
        log::debug!("Increasing voltage to {}mv", millivolts);
        // Safety: the possible values of millivolts after going through
        // reg3_trg fits in 5 bits.
        dcdc.reg3
            .modify(|_, w| unsafe { w.trg().bits(reg3_trg(millivolts)) });
        while dcdc.reg0.read().sts_dc_ok().bit_is_clear() {
            core::sync::atomic::spin_loop_hint();
        }
    }

    use pac::ccm::cbcdr::PERIPH_CLK_SEL_A;
    if PERIPH_CLK_SEL_A::PERIPH_CLK_SEL_0 == ccm.cbcdr.read().periph_clk_sel().variant() {
        log::debug!("Choosing alternative clock before reconfiguring ARM PLL...");
        use pac::ccm::{cbcdr::PERIPH_CLK2_PODF_A, cbcmr::PERIPH_CLK2_SEL_A};
        let pll_usb1 = ccm_analog.pll_usb1.read();
        let (sel, div) = if pll_usb1.enable().bit_is_set()
            && pll_usb1.power().bit_is_set()
            && pll_usb1.lock().bit_is_set()
            && pll_usb1.en_usb_clks().bit_is_set()
        {
            log::debug!("Using USB PLL, divided down to 120MHz");
            (
                PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_0,
                PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_3,
            )
        } else {
            log::debug!("USB PLL is off; using 24MHz oscillator");
            (
                PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_1,
                PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_0,
            )
        };
        ccm.cbcdr.modify(|_, w| w.periph_clk2_podf().variant(div));
        ccm.cbcmr.modify(|_, w| w.periph_clk2_sel().variant(sel));
        while ccm.cdhipr.read().periph2_clk_sel_busy().bit_is_set() {
            core::sync::atomic::spin_loop_hint();
        }
        ccm.cbcdr.modify(|_, w| w.periph_clk_sel().set_bit());
        while ccm.cdhipr.read().periph_clk_sel_busy().bit_is_set() {
            core::sync::atomic::spin_loop_hint();
        }
    } else {
        log::debug!("Already running from PERIPH2_CLK2");
    }

    let (mut div_arm, mut div_ahb) = (1, 1);
    while hz * div_arm * div_ahb < 648_000_000 {
        if div_arm < 8 {
            div_arm += 1;
        } else if div_ahb < 5 {
            div_ahb += 1;
            div_arm = 1;
        } else {
            break;
        }
    }
    let mult = (hz * div_arm * div_ahb + 6_000_000) / 12_000_000;
    let mult = mult.min(108).max(54);
    log::debug!(
        "Frequency 12MHz * {mult} / {div_arm} / {div_ahb}",
        mult = mult,
        div_arm = div_arm,
        div_ahb = div_ahb
    );
    hz = mult * 12_000_000 / div_arm / div_ahb;

    log::debug!("ARM PLL = 0x{:x}", ccm_analog.pll_arm.read().bits());
    ccm_analog
        .pll_arm
        .write_with_zero(|w| w.powerdown().set_bit());
    ccm_analog
        .pll_arm
        .write_with_zero(unsafe { |w| w.enable().set_bit().div_select().bits(mult as u8) });
    while ccm_analog.pll_arm.read().lock().bit_is_clear() {
        core::sync::atomic::spin_loop_hint();
    }
    log::debug!("ARM PLL = 0x{:x}", ccm_analog.pll_arm.read().bits());

    ccm.cacrr.write(|w| w.arm_podf().bits((div_arm - 1) as u8));
    while ccm.cdhipr.read().arm_podf_busy().bit_is_set() {
        core::sync::atomic::spin_loop_hint();
    }
    ccm.cbcdr
        .modify(|_, w| w.ahb_podf().bits((div_ahb - 1) as u8));
    while ccm.cdhipr.read().arm_podf_busy().bit_is_set() {
        core::sync::atomic::spin_loop_hint();
    }

    let div_ipg = (hz + 149_999_999) / 150_000_000;
    let div_ipg = div_ipg.min(4);
    ccm.cbcdr
        .modify(|_, w| w.ipg_podf().bits((div_ipg - 1) as u8));
    ccm.cbcdr.modify(|_, w| w.periph_clk_sel().clear_bit());
    while ccm.cdhipr.read().periph_clk_sel_busy().bit_is_set() {
        core::sync::atomic::spin_loop_hint();
    }

    log::debug!("ARM={}, IPG={}", hz, hz / div_ipg);
    if dcdc.reg3.read().trg().bits() > reg3_trg(millivolts) {
        log::debug!("Decreasing voltage to {}mv", millivolts);
        dcdc.reg3
            .modify(|_, w| unsafe { w.trg().bits(reg3_trg(millivolts)) });
        while dcdc.reg0.read().sts_dc_ok().bit_is_clear() {
            core::sync::atomic::spin_loop_hint();
        }
    }

    hz
}

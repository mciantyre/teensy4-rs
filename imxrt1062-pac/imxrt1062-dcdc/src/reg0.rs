#[doc = "Reader of register REG0"]
pub type R = crate::R<u32, super::REG0>;
#[doc = "Writer for register REG0"]
pub type W = crate::W<u32, super::REG0>;
#[doc = "Register REG0 `reset()`'s with value 0x1403_0111"]
impl crate::ResetValue for super::REG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1403_0111
    }
}
#[doc = "Reader of field `PWD_ZCD`"]
pub type PWD_ZCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWD_ZCD`"]
pub struct PWD_ZCD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWD_ZCD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DISABLE_AUTO_CLK_SWITCH`"]
pub type DISABLE_AUTO_CLK_SWITCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_AUTO_CLK_SWITCH`"]
pub struct DISABLE_AUTO_CLK_SWITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_AUTO_CLK_SWITCH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SEL_CLK`"]
pub type SEL_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEL_CLK`"]
pub struct SEL_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_CLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PWD_OSC_INT`"]
pub type PWD_OSC_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWD_OSC_INT`"]
pub struct PWD_OSC_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWD_OSC_INT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PWD_CUR_SNS_CMP`"]
pub type PWD_CUR_SNS_CMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWD_CUR_SNS_CMP`"]
pub struct PWD_CUR_SNS_CMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWD_CUR_SNS_CMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CUR_SNS_THRSH`"]
pub type CUR_SNS_THRSH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CUR_SNS_THRSH`"]
pub struct CUR_SNS_THRSH_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_SNS_THRSH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `PWD_OVERCUR_DET`"]
pub type PWD_OVERCUR_DET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWD_OVERCUR_DET`"]
pub struct PWD_OVERCUR_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> PWD_OVERCUR_DET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `OVERCUR_TRIG_ADJ`"]
pub type OVERCUR_TRIG_ADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OVERCUR_TRIG_ADJ`"]
pub struct OVERCUR_TRIG_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERCUR_TRIG_ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `PWD_CMP_BATT_DET`"]
pub type PWD_CMP_BATT_DET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWD_CMP_BATT_DET`"]
pub struct PWD_CMP_BATT_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> PWD_CMP_BATT_DET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ADJ_POSLIMIT_BUCK`"]
pub type ADJ_POSLIMIT_BUCK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADJ_POSLIMIT_BUCK`"]
pub struct ADJ_POSLIMIT_BUCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADJ_POSLIMIT_BUCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `EN_LP_OVERLOAD_SNS`"]
pub type EN_LP_OVERLOAD_SNS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_LP_OVERLOAD_SNS`"]
pub struct EN_LP_OVERLOAD_SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_LP_OVERLOAD_SNS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `PWD_HIGH_VOLT_DET`"]
pub type PWD_HIGH_VOLT_DET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWD_HIGH_VOLT_DET`"]
pub struct PWD_HIGH_VOLT_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> PWD_HIGH_VOLT_DET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `LP_OVERLOAD_THRSH`"]
pub type LP_OVERLOAD_THRSH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LP_OVERLOAD_THRSH`"]
pub struct LP_OVERLOAD_THRSH_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_OVERLOAD_THRSH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `LP_OVERLOAD_FREQ_SEL`"]
pub type LP_OVERLOAD_FREQ_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LP_OVERLOAD_FREQ_SEL`"]
pub struct LP_OVERLOAD_FREQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_OVERLOAD_FREQ_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `LP_HIGH_HYS`"]
pub type LP_HIGH_HYS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LP_HIGH_HYS`"]
pub struct LP_HIGH_HYS_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_HIGH_HYS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `PWD_CMP_OFFSET`"]
pub type PWD_CMP_OFFSET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWD_CMP_OFFSET`"]
pub struct PWD_CMP_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PWD_CMP_OFFSET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `XTALOK_DISABLE`"]
pub type XTALOK_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTALOK_DISABLE`"]
pub struct XTALOK_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALOK_DISABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `CURRENT_ALERT_RESET`"]
pub type CURRENT_ALERT_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CURRENT_ALERT_RESET`"]
pub struct CURRENT_ALERT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENT_ALERT_RESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `XTAL_24M_OK`"]
pub type XTAL_24M_OK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTAL_24M_OK`"]
pub struct XTAL_24M_OK_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_24M_OK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `STS_DC_OK`"]
pub type STS_DC_OK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - power down the zero cross detection function for discontinuous conductor mode"]
    #[inline(always)]
    pub fn pwd_zcd(&self) -> PWD_ZCD_R {
        PWD_ZCD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable automatic clock switch from internal osc to xtal clock."]
    #[inline(always)]
    pub fn disable_auto_clk_switch(&self) -> DISABLE_AUTO_CLK_SWITCH_R {
        DISABLE_AUTO_CLK_SWITCH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - select 24 MHz Crystal clock for DCDC, when dcdc_disable_auto_clk_switch is set."]
    #[inline(always)]
    pub fn sel_clk(&self) -> SEL_CLK_R {
        SEL_CLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Power down internal osc. Only set this bit, when 24 MHz crystal osc is available"]
    #[inline(always)]
    pub fn pwd_osc_int(&self) -> PWD_OSC_INT_R {
        PWD_OSC_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The power down signal of the current detector."]
    #[inline(always)]
    pub fn pwd_cur_sns_cmp(&self) -> PWD_CUR_SNS_CMP_R {
        PWD_CUR_SNS_CMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Set the threshold of current detector, if the peak current of the inductor exceeds the threshold, the current detector will assert"]
    #[inline(always)]
    pub fn cur_sns_thrsh(&self) -> CUR_SNS_THRSH_R {
        CUR_SNS_THRSH_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - power down overcurrent detection comparator"]
    #[inline(always)]
    pub fn pwd_overcur_det(&self) -> PWD_OVERCUR_DET_R {
        PWD_OVERCUR_DET_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - The threshold of over current detection in run mode and power save mode: run mode power save mode 0x0 1 A 0"]
    #[inline(always)]
    pub fn overcur_trig_adj(&self) -> OVERCUR_TRIG_ADJ_R {
        OVERCUR_TRIG_ADJ_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - set to \"1\" to power down the low voltage detection comparator"]
    #[inline(always)]
    pub fn pwd_cmp_batt_det(&self) -> PWD_CMP_BATT_DET_R {
        PWD_CMP_BATT_DET_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - adjust value to poslimit_buck register"]
    #[inline(always)]
    pub fn adj_poslimit_buck(&self) -> ADJ_POSLIMIT_BUCK_R {
        ADJ_POSLIMIT_BUCK_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - enable the overload detection in power save mode, if current is larger than the overloading threshold (typical value is 50 mA), DCDC will switch to the run mode automatically"]
    #[inline(always)]
    pub fn en_lp_overload_sns(&self) -> EN_LP_OVERLOAD_SNS_R {
        EN_LP_OVERLOAD_SNS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - power down overvoltage detection comparator"]
    #[inline(always)]
    pub fn pwd_high_volt_det(&self) -> PWD_HIGH_VOLT_DET_R {
        PWD_HIGH_VOLT_DET_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - the threshold of the counting number of charging times during the period that lp_overload_freq_sel sets in power save mode"]
    #[inline(always)]
    pub fn lp_overload_thrsh(&self) -> LP_OVERLOAD_THRSH_R {
        LP_OVERLOAD_THRSH_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - the period of counting the charging times in power save mode 0: eight 32k cycle 1: sixteen 32k cycle"]
    #[inline(always)]
    pub fn lp_overload_freq_sel(&self) -> LP_OVERLOAD_FREQ_SEL_R {
        LP_OVERLOAD_FREQ_SEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Adjust hysteretic value in low power from 12.5mV to 25mV"]
    #[inline(always)]
    pub fn lp_high_hys(&self) -> LP_HIGH_HYS_R {
        LP_HIGH_HYS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 26 - power down output range comparator"]
    #[inline(always)]
    pub fn pwd_cmp_offset(&self) -> PWD_CMP_OFFSET_R {
        PWD_CMP_OFFSET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 1'b1: Disable xtalok detection circuit 1'b0: Enable xtalok detection circuit"]
    #[inline(always)]
    pub fn xtalok_disable(&self) -> XTALOK_DISABLE_R {
        XTALOK_DISABLE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - reset current alert signal"]
    #[inline(always)]
    pub fn current_alert_reset(&self) -> CURRENT_ALERT_RESET_R {
        CURRENT_ALERT_RESET_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - set to 1 to switch internal ring osc to xtal 24M"]
    #[inline(always)]
    pub fn xtal_24m_ok(&self) -> XTAL_24M_OK_R {
        XTAL_24M_OK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Status register to indicate DCDC status. 1'b1: DCDC already settled 1'b0: DCDC is settling"]
    #[inline(always)]
    pub fn sts_dc_ok(&self) -> STS_DC_OK_R {
        STS_DC_OK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - power down the zero cross detection function for discontinuous conductor mode"]
    #[inline(always)]
    pub fn pwd_zcd(&mut self) -> PWD_ZCD_W {
        PWD_ZCD_W { w: self }
    }
    #[doc = "Bit 1 - Disable automatic clock switch from internal osc to xtal clock."]
    #[inline(always)]
    pub fn disable_auto_clk_switch(&mut self) -> DISABLE_AUTO_CLK_SWITCH_W {
        DISABLE_AUTO_CLK_SWITCH_W { w: self }
    }
    #[doc = "Bit 2 - select 24 MHz Crystal clock for DCDC, when dcdc_disable_auto_clk_switch is set."]
    #[inline(always)]
    pub fn sel_clk(&mut self) -> SEL_CLK_W {
        SEL_CLK_W { w: self }
    }
    #[doc = "Bit 3 - Power down internal osc. Only set this bit, when 24 MHz crystal osc is available"]
    #[inline(always)]
    pub fn pwd_osc_int(&mut self) -> PWD_OSC_INT_W {
        PWD_OSC_INT_W { w: self }
    }
    #[doc = "Bit 4 - The power down signal of the current detector."]
    #[inline(always)]
    pub fn pwd_cur_sns_cmp(&mut self) -> PWD_CUR_SNS_CMP_W {
        PWD_CUR_SNS_CMP_W { w: self }
    }
    #[doc = "Bits 5:7 - Set the threshold of current detector, if the peak current of the inductor exceeds the threshold, the current detector will assert"]
    #[inline(always)]
    pub fn cur_sns_thrsh(&mut self) -> CUR_SNS_THRSH_W {
        CUR_SNS_THRSH_W { w: self }
    }
    #[doc = "Bit 8 - power down overcurrent detection comparator"]
    #[inline(always)]
    pub fn pwd_overcur_det(&mut self) -> PWD_OVERCUR_DET_W {
        PWD_OVERCUR_DET_W { w: self }
    }
    #[doc = "Bits 9:10 - The threshold of over current detection in run mode and power save mode: run mode power save mode 0x0 1 A 0"]
    #[inline(always)]
    pub fn overcur_trig_adj(&mut self) -> OVERCUR_TRIG_ADJ_W {
        OVERCUR_TRIG_ADJ_W { w: self }
    }
    #[doc = "Bit 11 - set to \"1\" to power down the low voltage detection comparator"]
    #[inline(always)]
    pub fn pwd_cmp_batt_det(&mut self) -> PWD_CMP_BATT_DET_W {
        PWD_CMP_BATT_DET_W { w: self }
    }
    #[doc = "Bits 12:15 - adjust value to poslimit_buck register"]
    #[inline(always)]
    pub fn adj_poslimit_buck(&mut self) -> ADJ_POSLIMIT_BUCK_W {
        ADJ_POSLIMIT_BUCK_W { w: self }
    }
    #[doc = "Bit 16 - enable the overload detection in power save mode, if current is larger than the overloading threshold (typical value is 50 mA), DCDC will switch to the run mode automatically"]
    #[inline(always)]
    pub fn en_lp_overload_sns(&mut self) -> EN_LP_OVERLOAD_SNS_W {
        EN_LP_OVERLOAD_SNS_W { w: self }
    }
    #[doc = "Bit 17 - power down overvoltage detection comparator"]
    #[inline(always)]
    pub fn pwd_high_volt_det(&mut self) -> PWD_HIGH_VOLT_DET_W {
        PWD_HIGH_VOLT_DET_W { w: self }
    }
    #[doc = "Bits 18:19 - the threshold of the counting number of charging times during the period that lp_overload_freq_sel sets in power save mode"]
    #[inline(always)]
    pub fn lp_overload_thrsh(&mut self) -> LP_OVERLOAD_THRSH_W {
        LP_OVERLOAD_THRSH_W { w: self }
    }
    #[doc = "Bit 20 - the period of counting the charging times in power save mode 0: eight 32k cycle 1: sixteen 32k cycle"]
    #[inline(always)]
    pub fn lp_overload_freq_sel(&mut self) -> LP_OVERLOAD_FREQ_SEL_W {
        LP_OVERLOAD_FREQ_SEL_W { w: self }
    }
    #[doc = "Bit 21 - Adjust hysteretic value in low power from 12.5mV to 25mV"]
    #[inline(always)]
    pub fn lp_high_hys(&mut self) -> LP_HIGH_HYS_W {
        LP_HIGH_HYS_W { w: self }
    }
    #[doc = "Bit 26 - power down output range comparator"]
    #[inline(always)]
    pub fn pwd_cmp_offset(&mut self) -> PWD_CMP_OFFSET_W {
        PWD_CMP_OFFSET_W { w: self }
    }
    #[doc = "Bit 27 - 1'b1: Disable xtalok detection circuit 1'b0: Enable xtalok detection circuit"]
    #[inline(always)]
    pub fn xtalok_disable(&mut self) -> XTALOK_DISABLE_W {
        XTALOK_DISABLE_W { w: self }
    }
    #[doc = "Bit 28 - reset current alert signal"]
    #[inline(always)]
    pub fn current_alert_reset(&mut self) -> CURRENT_ALERT_RESET_W {
        CURRENT_ALERT_RESET_W { w: self }
    }
    #[doc = "Bit 29 - set to 1 to switch internal ring osc to xtal 24M"]
    #[inline(always)]
    pub fn xtal_24m_ok(&mut self) -> XTAL_24M_OK_W {
        XTAL_24M_OK_W { w: self }
    }
}

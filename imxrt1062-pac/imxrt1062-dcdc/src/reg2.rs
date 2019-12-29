#[doc = "Reader of register REG2"]
pub type R = crate::R<u32, super::REG2>;
#[doc = "Writer for register REG2"]
pub type W = crate::W<u32, super::REG2>;
#[doc = "Register REG2 `reset()`'s with value 0x4009"]
impl crate::ResetValue for super::REG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4009
    }
}
#[doc = "Reader of field `LOOPCTRL_DC_C`"]
pub type LOOPCTRL_DC_C_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOOPCTRL_DC_C`"]
pub struct LOOPCTRL_DC_C_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPCTRL_DC_C_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `LOOPCTRL_DC_R`"]
pub type LOOPCTRL_DC_R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOOPCTRL_DC_R`"]
pub struct LOOPCTRL_DC_R_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPCTRL_DC_R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `LOOPCTRL_DC_FF`"]
pub type LOOPCTRL_DC_FF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOOPCTRL_DC_FF`"]
pub struct LOOPCTRL_DC_FF_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPCTRL_DC_FF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `LOOPCTRL_EN_RCSCALE`"]
pub type LOOPCTRL_EN_RCSCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOOPCTRL_EN_RCSCALE`"]
pub struct LOOPCTRL_EN_RCSCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPCTRL_EN_RCSCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `LOOPCTRL_RCSCALE_THRSH`"]
pub type LOOPCTRL_RCSCALE_THRSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPCTRL_RCSCALE_THRSH`"]
pub struct LOOPCTRL_RCSCALE_THRSH_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPCTRL_RCSCALE_THRSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `LOOPCTRL_HYST_SIGN`"]
pub type LOOPCTRL_HYST_SIGN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPCTRL_HYST_SIGN`"]
pub struct LOOPCTRL_HYST_SIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPCTRL_HYST_SIGN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `DISABLE_PULSE_SKIP`"]
pub type DISABLE_PULSE_SKIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_PULSE_SKIP`"]
pub struct DISABLE_PULSE_SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_PULSE_SKIP_W<'a> {
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
#[doc = "Reader of field `DCM_SET_CTRL`"]
pub type DCM_SET_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCM_SET_CTRL`"]
pub struct DCM_SET_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCM_SET_CTRL_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Ratio of integral control parameter to proportional control parameter in the switching DC-DC converter, and can be used to optimize efficiency and loop response"]
    #[inline(always)]
    pub fn loopctrl_dc_c(&self) -> LOOPCTRL_DC_C_R {
        LOOPCTRL_DC_C_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:5 - Magnitude of proportional control parameter in the switching DC-DC converter control loop."]
    #[inline(always)]
    pub fn loopctrl_dc_r(&self) -> LOOPCTRL_DC_R_R {
        LOOPCTRL_DC_R_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:8 - Two's complement feed forward step in duty cycle in the switching DC-DC converter"]
    #[inline(always)]
    pub fn loopctrl_dc_ff(&self) -> LOOPCTRL_DC_FF_R {
        LOOPCTRL_DC_FF_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Enable analog circuit of DC-DC converter to respond faster under transient load conditions."]
    #[inline(always)]
    pub fn loopctrl_en_rcscale(&self) -> LOOPCTRL_EN_RCSCALE_R {
        LOOPCTRL_EN_RCSCALE_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Increase the threshold detection for RC scale circuit."]
    #[inline(always)]
    pub fn loopctrl_rcscale_thrsh(&self) -> LOOPCTRL_RCSCALE_THRSH_R {
        LOOPCTRL_RCSCALE_THRSH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Invert the sign of the hysteresis in DC-DC analog comparators."]
    #[inline(always)]
    pub fn loopctrl_hyst_sign(&self) -> LOOPCTRL_HYST_SIGN_R {
        LOOPCTRL_HYST_SIGN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Set to \"0\" : stop charging if the duty cycle is lower than what set by dcdc_neglimit_in"]
    #[inline(always)]
    pub fn disable_pulse_skip(&self) -> DISABLE_PULSE_SKIP_R {
        DISABLE_PULSE_SKIP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set high to improve the transition from heavy load to light load"]
    #[inline(always)]
    pub fn dcm_set_ctrl(&self) -> DCM_SET_CTRL_R {
        DCM_SET_CTRL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ratio of integral control parameter to proportional control parameter in the switching DC-DC converter, and can be used to optimize efficiency and loop response"]
    #[inline(always)]
    pub fn loopctrl_dc_c(&mut self) -> LOOPCTRL_DC_C_W {
        LOOPCTRL_DC_C_W { w: self }
    }
    #[doc = "Bits 2:5 - Magnitude of proportional control parameter in the switching DC-DC converter control loop."]
    #[inline(always)]
    pub fn loopctrl_dc_r(&mut self) -> LOOPCTRL_DC_R_W {
        LOOPCTRL_DC_R_W { w: self }
    }
    #[doc = "Bits 6:8 - Two's complement feed forward step in duty cycle in the switching DC-DC converter"]
    #[inline(always)]
    pub fn loopctrl_dc_ff(&mut self) -> LOOPCTRL_DC_FF_W {
        LOOPCTRL_DC_FF_W { w: self }
    }
    #[doc = "Bits 9:11 - Enable analog circuit of DC-DC converter to respond faster under transient load conditions."]
    #[inline(always)]
    pub fn loopctrl_en_rcscale(&mut self) -> LOOPCTRL_EN_RCSCALE_W {
        LOOPCTRL_EN_RCSCALE_W { w: self }
    }
    #[doc = "Bit 12 - Increase the threshold detection for RC scale circuit."]
    #[inline(always)]
    pub fn loopctrl_rcscale_thrsh(&mut self) -> LOOPCTRL_RCSCALE_THRSH_W {
        LOOPCTRL_RCSCALE_THRSH_W { w: self }
    }
    #[doc = "Bit 13 - Invert the sign of the hysteresis in DC-DC analog comparators."]
    #[inline(always)]
    pub fn loopctrl_hyst_sign(&mut self) -> LOOPCTRL_HYST_SIGN_W {
        LOOPCTRL_HYST_SIGN_W { w: self }
    }
    #[doc = "Bit 27 - Set to \"0\" : stop charging if the duty cycle is lower than what set by dcdc_neglimit_in"]
    #[inline(always)]
    pub fn disable_pulse_skip(&mut self) -> DISABLE_PULSE_SKIP_W {
        DISABLE_PULSE_SKIP_W { w: self }
    }
    #[doc = "Bit 28 - Set high to improve the transition from heavy load to light load"]
    #[inline(always)]
    pub fn dcm_set_ctrl(&mut self) -> DCM_SET_CTRL_W {
        DCM_SET_CTRL_W { w: self }
    }
}

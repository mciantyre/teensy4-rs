#[doc = "Reader of register REG1"]
pub type R = crate::R<u32, super::REG1>;
#[doc = "Writer for register REG1"]
pub type W = crate::W<u32, super::REG1>;
#[doc = "Register REG1 `reset()`'s with value 0x111b_a29c"]
impl crate::ResetValue for super::REG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x111b_a29c
    }
}
#[doc = "Reader of field `REG_FBK_SEL`"]
pub type REG_FBK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REG_FBK_SEL`"]
pub struct REG_FBK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_FBK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `REG_RLOAD_SW`"]
pub type REG_RLOAD_SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REG_RLOAD_SW`"]
pub struct REG_RLOAD_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_RLOAD_SW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `LP_CMP_ISRC_SEL`"]
pub type LP_CMP_ISRC_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LP_CMP_ISRC_SEL`"]
pub struct LP_CMP_ISRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_CMP_ISRC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `LOOPCTRL_HST_THRESH`"]
pub type LOOPCTRL_HST_THRESH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPCTRL_HST_THRESH`"]
pub struct LOOPCTRL_HST_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPCTRL_HST_THRESH_W<'a> {
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
#[doc = "Reader of field `LOOPCTRL_EN_HYST`"]
pub type LOOPCTRL_EN_HYST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPCTRL_EN_HYST`"]
pub struct LOOPCTRL_EN_HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPCTRL_EN_HYST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `VBG_TRIM`"]
pub type VBG_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VBG_TRIM`"]
pub struct VBG_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VBG_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:8 - select the feedback point of the internal regulator"]
    #[inline(always)]
    pub fn reg_fbk_sel(&self) -> REG_FBK_SEL_R {
        REG_FBK_SEL_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 9 - control the load resistor of the internal regulator of DCDC, the load resistor is connected as default \"1\", and need set to \"0\" to disconnect the load resistor"]
    #[inline(always)]
    pub fn reg_rload_sw(&self) -> REG_RLOAD_SW_R {
        REG_RLOAD_SW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - set the current bias of low power comparator 0x0: 50 nA 0x1: 100 nA 0x2: 200 nA 0x3: 400 nA"]
    #[inline(always)]
    pub fn lp_cmp_isrc_sel(&self) -> LP_CMP_ISRC_SEL_R {
        LP_CMP_ISRC_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 21 - increase the threshold detection for common mode analog comparator"]
    #[inline(always)]
    pub fn loopctrl_hst_thresh(&self) -> LOOPCTRL_HST_THRESH_R {
        LOOPCTRL_HST_THRESH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable hysteresis in switching converter common mode analog comparators"]
    #[inline(always)]
    pub fn loopctrl_en_hyst(&self) -> LOOPCTRL_EN_HYST_R {
        LOOPCTRL_EN_HYST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - trim bandgap voltage"]
    #[inline(always)]
    pub fn vbg_trim(&self) -> VBG_TRIM_R {
        VBG_TRIM_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:8 - select the feedback point of the internal regulator"]
    #[inline(always)]
    pub fn reg_fbk_sel(&mut self) -> REG_FBK_SEL_W {
        REG_FBK_SEL_W { w: self }
    }
    #[doc = "Bit 9 - control the load resistor of the internal regulator of DCDC, the load resistor is connected as default \"1\", and need set to \"0\" to disconnect the load resistor"]
    #[inline(always)]
    pub fn reg_rload_sw(&mut self) -> REG_RLOAD_SW_W {
        REG_RLOAD_SW_W { w: self }
    }
    #[doc = "Bits 12:13 - set the current bias of low power comparator 0x0: 50 nA 0x1: 100 nA 0x2: 200 nA 0x3: 400 nA"]
    #[inline(always)]
    pub fn lp_cmp_isrc_sel(&mut self) -> LP_CMP_ISRC_SEL_W {
        LP_CMP_ISRC_SEL_W { w: self }
    }
    #[doc = "Bit 21 - increase the threshold detection for common mode analog comparator"]
    #[inline(always)]
    pub fn loopctrl_hst_thresh(&mut self) -> LOOPCTRL_HST_THRESH_W {
        LOOPCTRL_HST_THRESH_W { w: self }
    }
    #[doc = "Bit 23 - Enable hysteresis in switching converter common mode analog comparators"]
    #[inline(always)]
    pub fn loopctrl_en_hyst(&mut self) -> LOOPCTRL_EN_HYST_W {
        LOOPCTRL_EN_HYST_W { w: self }
    }
    #[doc = "Bits 24:28 - trim bandgap voltage"]
    #[inline(always)]
    pub fn vbg_trim(&mut self) -> VBG_TRIM_W {
        VBG_TRIM_W { w: self }
    }
}

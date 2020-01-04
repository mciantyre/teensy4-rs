#[doc = "Reader of register GPR3"]
pub type R = crate::R<u32, super::GPR3>;
#[doc = "Writer for register GPR3"]
pub type W = crate::W<u32, super::GPR3>;
#[doc = "Register GPR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPSR_MODE_ENABLE`"]
pub type LPSR_MODE_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPSR_MODE_ENABLE`"]
pub struct LPSR_MODE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSR_MODE_ENABLE_W<'a> {
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
#[doc = "Reader of field `DCDC_STATUS_CAPT_CLR`"]
pub type DCDC_STATUS_CAPT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_STATUS_CAPT_CLR`"]
pub struct DCDC_STATUS_CAPT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_STATUS_CAPT_CLR_W<'a> {
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
#[doc = "Reader of field `POR_PULL_TYPE`"]
pub type POR_PULL_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POR_PULL_TYPE`"]
pub struct POR_PULL_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_PULL_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `DCDC_IN_LOW_VOL`"]
pub type DCDC_IN_LOW_VOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCDC_OVER_CUR`"]
pub type DCDC_OVER_CUR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCDC_OVER_VOL`"]
pub type DCDC_OVER_VOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCDC_STS_DC_OK`"]
pub type DCDC_STS_DC_OK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Set to enable LPSR mode."]
    #[inline(always)]
    pub fn lpsr_mode_enable(&self) -> LPSR_MODE_ENABLE_R {
        LPSR_MODE_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DCDC captured status clear"]
    #[inline(always)]
    pub fn dcdc_status_capt_clr(&self) -> DCDC_STATUS_CAPT_CLR_R {
        DCDC_STATUS_CAPT_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - POR_B pad control"]
    #[inline(always)]
    pub fn por_pull_type(&self) -> POR_PULL_TYPE_R {
        POR_PULL_TYPE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 16 - DCDC_IN low voltage detect."]
    #[inline(always)]
    pub fn dcdc_in_low_vol(&self) -> DCDC_IN_LOW_VOL_R {
        DCDC_IN_LOW_VOL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DCDC output over current alert"]
    #[inline(always)]
    pub fn dcdc_over_cur(&self) -> DCDC_OVER_CUR_R {
        DCDC_OVER_CUR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DCDC output over voltage alert"]
    #[inline(always)]
    pub fn dcdc_over_vol(&self) -> DCDC_OVER_VOL_R {
        DCDC_OVER_VOL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DCDC status OK"]
    #[inline(always)]
    pub fn dcdc_sts_dc_ok(&self) -> DCDC_STS_DC_OK_R {
        DCDC_STS_DC_OK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable LPSR mode."]
    #[inline(always)]
    pub fn lpsr_mode_enable(&mut self) -> LPSR_MODE_ENABLE_W {
        LPSR_MODE_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - DCDC captured status clear"]
    #[inline(always)]
    pub fn dcdc_status_capt_clr(&mut self) -> DCDC_STATUS_CAPT_CLR_W {
        DCDC_STATUS_CAPT_CLR_W { w: self }
    }
    #[doc = "Bits 2:3 - POR_B pad control"]
    #[inline(always)]
    pub fn por_pull_type(&mut self) -> POR_PULL_TYPE_W {
        POR_PULL_TYPE_W { w: self }
    }
}

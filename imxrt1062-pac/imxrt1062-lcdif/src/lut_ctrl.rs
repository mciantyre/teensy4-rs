#[doc = "Reader of register LUT_CTRL"]
pub type R = crate::R<u32, super::LUT_CTRL>;
#[doc = "Writer for register LUT_CTRL"]
pub type W = crate::W<u32, super::LUT_CTRL>;
#[doc = "Register LUT_CTRL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::LUT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `LUT_BYPASS`"]
pub type LUT_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LUT_BYPASS`"]
pub struct LUT_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_BYPASS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Setting this bit will bypass the LUT memory resource completely"]
    #[inline(always)]
    pub fn lut_bypass(&self) -> LUT_BYPASS_R {
        LUT_BYPASS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit will bypass the LUT memory resource completely"]
    #[inline(always)]
    pub fn lut_bypass(&mut self) -> LUT_BYPASS_W {
        LUT_BYPASS_W { w: self }
    }
}

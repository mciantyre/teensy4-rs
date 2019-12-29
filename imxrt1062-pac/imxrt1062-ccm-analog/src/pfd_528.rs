#[doc = "Reader of register PFD_528"]
pub type R = crate::R<u32, super::PFD_528>;
#[doc = "Writer for register PFD_528"]
pub type W = crate::W<u32, super::PFD_528>;
#[doc = "Register PFD_528 `reset()`'s with value 0x1018_101b"]
impl crate::ResetValue for super::PFD_528 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1018_101b
    }
}
#[doc = "Reader of field `PFD0_FRAC`"]
pub type PFD0_FRAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PFD0_FRAC`"]
pub struct PFD0_FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD0_FRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `PFD0_STABLE`"]
pub type PFD0_STABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PFD0_CLKGATE`"]
pub type PFD0_CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFD0_CLKGATE`"]
pub struct PFD0_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD0_CLKGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PFD1_FRAC`"]
pub type PFD1_FRAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PFD1_FRAC`"]
pub struct PFD1_FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD1_FRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PFD1_STABLE`"]
pub type PFD1_STABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PFD1_CLKGATE`"]
pub type PFD1_CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFD1_CLKGATE`"]
pub struct PFD1_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD1_CLKGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `PFD2_FRAC`"]
pub type PFD2_FRAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PFD2_FRAC`"]
pub struct PFD2_FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD2_FRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PFD2_STABLE`"]
pub type PFD2_STABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PFD2_CLKGATE`"]
pub type PFD2_CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFD2_CLKGATE`"]
pub struct PFD2_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD2_CLKGATE_W<'a> {
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
#[doc = "Reader of field `PFD3_FRAC`"]
pub type PFD3_FRAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PFD3_FRAC`"]
pub struct PFD3_FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD3_FRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PFD3_STABLE`"]
pub type PFD3_STABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PFD3_CLKGATE`"]
pub type PFD3_CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFD3_CLKGATE`"]
pub struct PFD3_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD3_CLKGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - This field controls the fractional divide value"]
    #[inline(always)]
    pub fn pfd0_frac(&self) -> PFD0_FRAC_R {
        PFD0_FRAC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub fn pfd0_stable(&self) -> PFD0_STABLE_R {
        PFD0_STABLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[inline(always)]
    pub fn pfd0_clkgate(&self) -> PFD0_CLKGATE_R {
        PFD0_CLKGATE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - This field controls the fractional divide value"]
    #[inline(always)]
    pub fn pfd1_frac(&self) -> PFD1_FRAC_R {
        PFD1_FRAC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub fn pfd1_stable(&self) -> PFD1_STABLE_R {
        PFD1_STABLE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IO Clock Gate"]
    #[inline(always)]
    pub fn pfd1_clkgate(&self) -> PFD1_CLKGATE_R {
        PFD1_CLKGATE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - This field controls the fractional divide value"]
    #[inline(always)]
    pub fn pfd2_frac(&self) -> PFD2_FRAC_R {
        PFD2_FRAC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub fn pfd2_stable(&self) -> PFD2_STABLE_R {
        PFD2_STABLE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - IO Clock Gate"]
    #[inline(always)]
    pub fn pfd2_clkgate(&self) -> PFD2_CLKGATE_R {
        PFD2_CLKGATE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - This field controls the fractional divide value"]
    #[inline(always)]
    pub fn pfd3_frac(&self) -> PFD3_FRAC_R {
        PFD3_FRAC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub fn pfd3_stable(&self) -> PFD3_STABLE_R {
        PFD3_STABLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IO Clock Gate"]
    #[inline(always)]
    pub fn pfd3_clkgate(&self) -> PFD3_CLKGATE_R {
        PFD3_CLKGATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - This field controls the fractional divide value"]
    #[inline(always)]
    pub fn pfd0_frac(&mut self) -> PFD0_FRAC_W {
        PFD0_FRAC_W { w: self }
    }
    #[doc = "Bit 7 - If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[inline(always)]
    pub fn pfd0_clkgate(&mut self) -> PFD0_CLKGATE_W {
        PFD0_CLKGATE_W { w: self }
    }
    #[doc = "Bits 8:13 - This field controls the fractional divide value"]
    #[inline(always)]
    pub fn pfd1_frac(&mut self) -> PFD1_FRAC_W {
        PFD1_FRAC_W { w: self }
    }
    #[doc = "Bit 15 - IO Clock Gate"]
    #[inline(always)]
    pub fn pfd1_clkgate(&mut self) -> PFD1_CLKGATE_W {
        PFD1_CLKGATE_W { w: self }
    }
    #[doc = "Bits 16:21 - This field controls the fractional divide value"]
    #[inline(always)]
    pub fn pfd2_frac(&mut self) -> PFD2_FRAC_W {
        PFD2_FRAC_W { w: self }
    }
    #[doc = "Bit 23 - IO Clock Gate"]
    #[inline(always)]
    pub fn pfd2_clkgate(&mut self) -> PFD2_CLKGATE_W {
        PFD2_CLKGATE_W { w: self }
    }
    #[doc = "Bits 24:29 - This field controls the fractional divide value"]
    #[inline(always)]
    pub fn pfd3_frac(&mut self) -> PFD3_FRAC_W {
        PFD3_FRAC_W { w: self }
    }
    #[doc = "Bit 31 - IO Clock Gate"]
    #[inline(always)]
    pub fn pfd3_clkgate(&mut self) -> PFD3_CLKGATE_W {
        PFD3_CLKGATE_W { w: self }
    }
}

#[doc = "Reader of register CSC1_COEF0"]
pub type R = crate::R<u32, super::CSC1_COEF0>;
#[doc = "Writer for register CSC1_COEF0"]
pub type W = crate::W<u32, super::CSC1_COEF0>;
#[doc = "Register CSC1_COEF0 `reset()`'s with value 0x0400_0000"]
impl crate::ResetValue for super::CSC1_COEF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400_0000
    }
}
#[doc = "Reader of field `Y_OFFSET`"]
pub type Y_OFFSET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Y_OFFSET`"]
pub struct Y_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> Y_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `UV_OFFSET`"]
pub type UV_OFFSET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UV_OFFSET`"]
pub struct UV_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> UV_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 9)) | (((value as u32) & 0x01ff) << 9);
        self.w
    }
}
#[doc = "Reader of field `C0`"]
pub type C0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `C0`"]
pub struct C0_W<'a> {
    w: &'a mut W,
}
impl<'a> C0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 18)) | (((value as u32) & 0x07ff) << 18);
        self.w
    }
}
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<bool, bool>;
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `YCBCR_MODE`"]
pub type YCBCR_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `YCBCR_MODE`"]
pub struct YCBCR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> YCBCR_MODE_W<'a> {
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
    #[doc = "Bits 0:8 - Two's compliment amplitude offset implicit in the Y data"]
    #[inline(always)]
    pub fn y_offset(&self) -> Y_OFFSET_R {
        Y_OFFSET_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - Two's compliment phase offset implicit for CbCr data"]
    #[inline(always)]
    pub fn uv_offset(&self) -> UV_OFFSET_R {
        UV_OFFSET_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:28 - Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    #[inline(always)]
    pub fn c0(&self) -> C0_R {
        C0_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Bypass the CSC unit in the scaling engine"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Set to 1 when performing YCbCr conversion to RGB"]
    #[inline(always)]
    pub fn ycbcr_mode(&self) -> YCBCR_MODE_R {
        YCBCR_MODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Two's compliment amplitude offset implicit in the Y data"]
    #[inline(always)]
    pub fn y_offset(&mut self) -> Y_OFFSET_W {
        Y_OFFSET_W { w: self }
    }
    #[doc = "Bits 9:17 - Two's compliment phase offset implicit for CbCr data"]
    #[inline(always)]
    pub fn uv_offset(&mut self) -> UV_OFFSET_W {
        UV_OFFSET_W { w: self }
    }
    #[doc = "Bits 18:28 - Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    #[inline(always)]
    pub fn c0(&mut self) -> C0_W {
        C0_W { w: self }
    }
    #[doc = "Bit 30 - Bypass the CSC unit in the scaling engine"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 31 - Set to 1 when performing YCbCr conversion to RGB"]
    #[inline(always)]
    pub fn ycbcr_mode(&mut self) -> YCBCR_MODE_W {
        YCBCR_MODE_W { w: self }
    }
}

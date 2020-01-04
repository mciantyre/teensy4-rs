#[doc = "Reader of register DLLCR%s"]
pub type R = crate::R<u32, super::DLLCR>;
#[doc = "Writer for register DLLCR%s"]
pub type W = crate::W<u32, super::DLLCR>;
#[doc = "Register DLLCR%s `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::DLLCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `DLLEN`"]
pub type DLLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLLEN`"]
pub struct DLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLEN_W<'a> {
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
#[doc = "Reader of field `DLLRESET`"]
pub type DLLRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLLRESET`"]
pub struct DLLRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLRESET_W<'a> {
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
#[doc = "Reader of field `SLVDLYTARGET`"]
pub type SLVDLYTARGET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLVDLYTARGET`"]
pub struct SLVDLYTARGET_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVDLYTARGET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = "Reader of field `OVRDEN`"]
pub type OVRDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVRDEN`"]
pub struct OVRDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRDEN_W<'a> {
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
#[doc = "Reader of field `OVRDVAL`"]
pub type OVRDVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OVRDVAL`"]
pub struct OVRDVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRDVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 9)) | (((value as u32) & 0x3f) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DLL calibration enable."]
    #[inline(always)]
    pub fn dllen(&self) -> DLLEN_R {
        DLLEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
    #[inline(always)]
    pub fn dllreset(&self) -> DLLRESET_R {
        DLLRESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial clock)."]
    #[inline(always)]
    pub fn slvdlytarget(&self) -> SLVDLYTARGET_R {
        SLVDLYTARGET_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Slave clock delay line delay cell number selection override enable."]
    #[inline(always)]
    pub fn ovrden(&self) -> OVRDEN_R {
        OVRDEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:14 - Slave clock delay line delay cell number selection override value."]
    #[inline(always)]
    pub fn ovrdval(&self) -> OVRDVAL_R {
        OVRDVAL_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DLL calibration enable."]
    #[inline(always)]
    pub fn dllen(&mut self) -> DLLEN_W {
        DLLEN_W { w: self }
    }
    #[doc = "Bit 1 - Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
    #[inline(always)]
    pub fn dllreset(&mut self) -> DLLRESET_W {
        DLLRESET_W { w: self }
    }
    #[doc = "Bits 3:6 - The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial clock)."]
    #[inline(always)]
    pub fn slvdlytarget(&mut self) -> SLVDLYTARGET_W {
        SLVDLYTARGET_W { w: self }
    }
    #[doc = "Bit 8 - Slave clock delay line delay cell number selection override enable."]
    #[inline(always)]
    pub fn ovrden(&mut self) -> OVRDEN_W {
        OVRDEN_W { w: self }
    }
    #[doc = "Bits 9:14 - Slave clock delay line delay cell number selection override value."]
    #[inline(always)]
    pub fn ovrdval(&mut self) -> OVRDVAL_W {
        OVRDVAL_W { w: self }
    }
}

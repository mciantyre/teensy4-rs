#[doc = "Reader of register AHBRXBUF1CR0"]
pub type R = crate::R<u32, super::AHBRXBUF1CR0>;
#[doc = "Writer for register AHBRXBUF1CR0"]
pub type W = crate::W<u32, super::AHBRXBUF1CR0>;
#[doc = "Register AHBRXBUF1CR0 `reset()`'s with value 0x8001_0020"]
impl crate::ResetValue for super::AHBRXBUF1CR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8001_0020
    }
}
#[doc = "Reader of field `BUFSZ`"]
pub type BUFSZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUFSZ`"]
pub struct BUFSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `MSTRID`"]
pub type MSTRID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSTRID`"]
pub struct MSTRID_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTRID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PRIORITY`"]
pub type PRIORITY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIORITY`"]
pub struct PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `PREFETCHEN`"]
pub type PREFETCHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREFETCHEN`"]
pub struct PREFETCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFETCHEN_W<'a> {
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
    #[doc = "Bits 0:7 - AHB RX Buffer Size in 64 bits.Refer AHB RX Buffer Management for more details."]
    #[inline(always)]
    pub fn bufsz(&self) -> BUFSZ_R {
        BUFSZ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID). Please refer to for AHB RX Buffer allocation."]
    #[inline(always)]
    pub fn mstrid(&self) -> MSTRID_R {
        MSTRID_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - This priority for AHB Master Read which this AHB RX Buffer is assigned. Refer for more details."]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 31 - AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub fn prefetchen(&self) -> PREFETCHEN_R {
        PREFETCHEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - AHB RX Buffer Size in 64 bits.Refer AHB RX Buffer Management for more details."]
    #[inline(always)]
    pub fn bufsz(&mut self) -> BUFSZ_W {
        BUFSZ_W { w: self }
    }
    #[doc = "Bits 16:19 - This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID). Please refer to for AHB RX Buffer allocation."]
    #[inline(always)]
    pub fn mstrid(&mut self) -> MSTRID_W {
        MSTRID_W { w: self }
    }
    #[doc = "Bits 24:25 - This priority for AHB Master Read which this AHB RX Buffer is assigned. Refer for more details."]
    #[inline(always)]
    pub fn priority(&mut self) -> PRIORITY_W {
        PRIORITY_W { w: self }
    }
    #[doc = "Bit 31 - AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub fn prefetchen(&mut self) -> PREFETCHEN_W {
        PREFETCHEN_W { w: self }
    }
}

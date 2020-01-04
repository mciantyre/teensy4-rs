#[doc = "Reader of register MB2_64B_WORD4"]
pub type R = crate::R<u32, super::MB2_64B_WORD4>;
#[doc = "Writer for register MB2_64B_WORD4"]
pub type W = crate::W<u32, super::MB2_64B_WORD4>;
#[doc = "Register MB2_64B_WORD4 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB2_64B_WORD4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_19`"]
pub type DATA_BYTE_19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_19`"]
pub struct DATA_BYTE_19_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_18`"]
pub type DATA_BYTE_18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_18`"]
pub struct DATA_BYTE_18_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_17`"]
pub type DATA_BYTE_17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_17`"]
pub struct DATA_BYTE_17_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_16`"]
pub type DATA_BYTE_16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_16`"]
pub struct DATA_BYTE_16_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_19(&self) -> DATA_BYTE_19_R {
        DATA_BYTE_19_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_18(&self) -> DATA_BYTE_18_R {
        DATA_BYTE_18_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_17(&self) -> DATA_BYTE_17_R {
        DATA_BYTE_17_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_16(&self) -> DATA_BYTE_16_R {
        DATA_BYTE_16_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_19(&mut self) -> DATA_BYTE_19_W {
        DATA_BYTE_19_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_18(&mut self) -> DATA_BYTE_18_W {
        DATA_BYTE_18_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_17(&mut self) -> DATA_BYTE_17_W {
        DATA_BYTE_17_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_16(&mut self) -> DATA_BYTE_16_W {
        DATA_BYTE_16_W { w: self }
    }
}

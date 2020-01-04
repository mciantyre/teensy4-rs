#[doc = "Reader of register MB14_16B_WORD1"]
pub type R = crate::R<u32, super::MB14_16B_WORD1>;
#[doc = "Writer for register MB14_16B_WORD1"]
pub type W = crate::W<u32, super::MB14_16B_WORD1>;
#[doc = "Register MB14_16B_WORD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB14_16B_WORD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_7`"]
pub type DATA_BYTE_7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_7`"]
pub struct DATA_BYTE_7_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_6`"]
pub type DATA_BYTE_6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_6`"]
pub struct DATA_BYTE_6_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_5`"]
pub type DATA_BYTE_5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_5`"]
pub struct DATA_BYTE_5_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_4`"]
pub type DATA_BYTE_4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_4`"]
pub struct DATA_BYTE_4_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_4_W<'a> {
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
    pub fn data_byte_7(&self) -> DATA_BYTE_7_R {
        DATA_BYTE_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_6(&self) -> DATA_BYTE_6_R {
        DATA_BYTE_6_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_5(&self) -> DATA_BYTE_5_R {
        DATA_BYTE_5_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_4(&self) -> DATA_BYTE_4_R {
        DATA_BYTE_4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_7(&mut self) -> DATA_BYTE_7_W {
        DATA_BYTE_7_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_6(&mut self) -> DATA_BYTE_6_W {
        DATA_BYTE_6_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_5(&mut self) -> DATA_BYTE_5_W {
        DATA_BYTE_5_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_4(&mut self) -> DATA_BYTE_4_W {
        DATA_BYTE_4_W { w: self }
    }
}

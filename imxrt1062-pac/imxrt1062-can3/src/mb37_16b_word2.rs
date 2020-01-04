#[doc = "Reader of register MB37_16B_WORD2"]
pub type R = crate::R<u32, super::MB37_16B_WORD2>;
#[doc = "Writer for register MB37_16B_WORD2"]
pub type W = crate::W<u32, super::MB37_16B_WORD2>;
#[doc = "Register MB37_16B_WORD2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB37_16B_WORD2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_11`"]
pub type DATA_BYTE_11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_11`"]
pub struct DATA_BYTE_11_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_10`"]
pub type DATA_BYTE_10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_10`"]
pub struct DATA_BYTE_10_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_9`"]
pub type DATA_BYTE_9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_9`"]
pub struct DATA_BYTE_9_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_8`"]
pub type DATA_BYTE_8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_8`"]
pub struct DATA_BYTE_8_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_8_W<'a> {
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
    pub fn data_byte_11(&self) -> DATA_BYTE_11_R {
        DATA_BYTE_11_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_10(&self) -> DATA_BYTE_10_R {
        DATA_BYTE_10_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_9(&self) -> DATA_BYTE_9_R {
        DATA_BYTE_9_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_8(&self) -> DATA_BYTE_8_R {
        DATA_BYTE_8_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_11(&mut self) -> DATA_BYTE_11_W {
        DATA_BYTE_11_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_10(&mut self) -> DATA_BYTE_10_W {
        DATA_BYTE_10_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_9(&mut self) -> DATA_BYTE_9_W {
        DATA_BYTE_9_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_8(&mut self) -> DATA_BYTE_8_W {
        DATA_BYTE_8_W { w: self }
    }
}

#[doc = "Reader of register MB33_16B_WORD3"]
pub type R = crate::R<u32, super::MB33_16B_WORD3>;
#[doc = "Writer for register MB33_16B_WORD3"]
pub type W = crate::W<u32, super::MB33_16B_WORD3>;
#[doc = "Register MB33_16B_WORD3 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB33_16B_WORD3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_15`"]
pub type DATA_BYTE_15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_15`"]
pub struct DATA_BYTE_15_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_14`"]
pub type DATA_BYTE_14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_14`"]
pub struct DATA_BYTE_14_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_13`"]
pub type DATA_BYTE_13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_13`"]
pub struct DATA_BYTE_13_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_12`"]
pub type DATA_BYTE_12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_12`"]
pub struct DATA_BYTE_12_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_12_W<'a> {
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
    pub fn data_byte_15(&self) -> DATA_BYTE_15_R {
        DATA_BYTE_15_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_14(&self) -> DATA_BYTE_14_R {
        DATA_BYTE_14_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_13(&self) -> DATA_BYTE_13_R {
        DATA_BYTE_13_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_12(&self) -> DATA_BYTE_12_R {
        DATA_BYTE_12_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_15(&mut self) -> DATA_BYTE_15_W {
        DATA_BYTE_15_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_14(&mut self) -> DATA_BYTE_14_W {
        DATA_BYTE_14_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_13(&mut self) -> DATA_BYTE_13_W {
        DATA_BYTE_13_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_12(&mut self) -> DATA_BYTE_12_W {
        DATA_BYTE_12_W { w: self }
    }
}

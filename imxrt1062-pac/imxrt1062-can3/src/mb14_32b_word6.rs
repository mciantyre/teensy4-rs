#[doc = "Reader of register MB14_32B_WORD6"]
pub type R = crate::R<u32, super::MB14_32B_WORD6>;
#[doc = "Writer for register MB14_32B_WORD6"]
pub type W = crate::W<u32, super::MB14_32B_WORD6>;
#[doc = "Register MB14_32B_WORD6 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB14_32B_WORD6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_27`"]
pub type DATA_BYTE_27_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_27`"]
pub struct DATA_BYTE_27_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_26`"]
pub type DATA_BYTE_26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_26`"]
pub struct DATA_BYTE_26_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_25`"]
pub type DATA_BYTE_25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_25`"]
pub struct DATA_BYTE_25_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_24`"]
pub type DATA_BYTE_24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_24`"]
pub struct DATA_BYTE_24_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_24_W<'a> {
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
    pub fn data_byte_27(&self) -> DATA_BYTE_27_R {
        DATA_BYTE_27_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_26(&self) -> DATA_BYTE_26_R {
        DATA_BYTE_26_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_25(&self) -> DATA_BYTE_25_R {
        DATA_BYTE_25_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_24(&self) -> DATA_BYTE_24_R {
        DATA_BYTE_24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_27(&mut self) -> DATA_BYTE_27_W {
        DATA_BYTE_27_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_26(&mut self) -> DATA_BYTE_26_W {
        DATA_BYTE_26_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_25(&mut self) -> DATA_BYTE_25_W {
        DATA_BYTE_25_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_24(&mut self) -> DATA_BYTE_24_W {
        DATA_BYTE_24_W { w: self }
    }
}

#[doc = "Reader of register MB10_64B_WORD7"]
pub type R = crate::R<u32, super::MB10_64B_WORD7>;
#[doc = "Writer for register MB10_64B_WORD7"]
pub type W = crate::W<u32, super::MB10_64B_WORD7>;
#[doc = "Register MB10_64B_WORD7 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB10_64B_WORD7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_31`"]
pub type DATA_BYTE_31_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_31`"]
pub struct DATA_BYTE_31_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_30`"]
pub type DATA_BYTE_30_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_30`"]
pub struct DATA_BYTE_30_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_29`"]
pub type DATA_BYTE_29_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_29`"]
pub struct DATA_BYTE_29_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_28`"]
pub type DATA_BYTE_28_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_28`"]
pub struct DATA_BYTE_28_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_28_W<'a> {
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
    pub fn data_byte_31(&self) -> DATA_BYTE_31_R {
        DATA_BYTE_31_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_30(&self) -> DATA_BYTE_30_R {
        DATA_BYTE_30_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_29(&self) -> DATA_BYTE_29_R {
        DATA_BYTE_29_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_28(&self) -> DATA_BYTE_28_R {
        DATA_BYTE_28_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_31(&mut self) -> DATA_BYTE_31_W {
        DATA_BYTE_31_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_30(&mut self) -> DATA_BYTE_30_W {
        DATA_BYTE_30_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_29(&mut self) -> DATA_BYTE_29_W {
        DATA_BYTE_29_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_28(&mut self) -> DATA_BYTE_28_W {
        DATA_BYTE_28_W { w: self }
    }
}

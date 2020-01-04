#[doc = "Reader of register MB6_64B_WORD10"]
pub type R = crate::R<u32, super::MB6_64B_WORD10>;
#[doc = "Writer for register MB6_64B_WORD10"]
pub type W = crate::W<u32, super::MB6_64B_WORD10>;
#[doc = "Register MB6_64B_WORD10 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB6_64B_WORD10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_43`"]
pub type DATA_BYTE_43_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_43`"]
pub struct DATA_BYTE_43_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_43_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_42`"]
pub type DATA_BYTE_42_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_42`"]
pub struct DATA_BYTE_42_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_42_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_41`"]
pub type DATA_BYTE_41_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_41`"]
pub struct DATA_BYTE_41_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_41_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_40`"]
pub type DATA_BYTE_40_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_40`"]
pub struct DATA_BYTE_40_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_40_W<'a> {
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
    pub fn data_byte_43(&self) -> DATA_BYTE_43_R {
        DATA_BYTE_43_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_42(&self) -> DATA_BYTE_42_R {
        DATA_BYTE_42_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_41(&self) -> DATA_BYTE_41_R {
        DATA_BYTE_41_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_40(&self) -> DATA_BYTE_40_R {
        DATA_BYTE_40_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_43(&mut self) -> DATA_BYTE_43_W {
        DATA_BYTE_43_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_42(&mut self) -> DATA_BYTE_42_W {
        DATA_BYTE_42_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_41(&mut self) -> DATA_BYTE_41_W {
        DATA_BYTE_41_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_40(&mut self) -> DATA_BYTE_40_W {
        DATA_BYTE_40_W { w: self }
    }
}

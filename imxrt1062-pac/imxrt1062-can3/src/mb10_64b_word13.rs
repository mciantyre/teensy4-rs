#[doc = "Reader of register MB10_64B_WORD13"]
pub type R = crate::R<u32, super::MB10_64B_WORD13>;
#[doc = "Writer for register MB10_64B_WORD13"]
pub type W = crate::W<u32, super::MB10_64B_WORD13>;
#[doc = "Register MB10_64B_WORD13 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB10_64B_WORD13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_55`"]
pub type DATA_BYTE_55_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_55`"]
pub struct DATA_BYTE_55_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_55_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_54`"]
pub type DATA_BYTE_54_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_54`"]
pub struct DATA_BYTE_54_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_54_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_53`"]
pub type DATA_BYTE_53_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_53`"]
pub struct DATA_BYTE_53_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_53_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_52`"]
pub type DATA_BYTE_52_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_52`"]
pub struct DATA_BYTE_52_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_52_W<'a> {
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
    pub fn data_byte_55(&self) -> DATA_BYTE_55_R {
        DATA_BYTE_55_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_54(&self) -> DATA_BYTE_54_R {
        DATA_BYTE_54_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_53(&self) -> DATA_BYTE_53_R {
        DATA_BYTE_53_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_52(&self) -> DATA_BYTE_52_R {
        DATA_BYTE_52_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_55(&mut self) -> DATA_BYTE_55_W {
        DATA_BYTE_55_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_54(&mut self) -> DATA_BYTE_54_W {
        DATA_BYTE_54_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_53(&mut self) -> DATA_BYTE_53_W {
        DATA_BYTE_53_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_52(&mut self) -> DATA_BYTE_52_W {
        DATA_BYTE_52_W { w: self }
    }
}

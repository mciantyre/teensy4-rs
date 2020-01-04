#[doc = "Reader of register MB5_32B_WORD5"]
pub type R = crate::R<u32, super::MB5_32B_WORD5>;
#[doc = "Writer for register MB5_32B_WORD5"]
pub type W = crate::W<u32, super::MB5_32B_WORD5>;
#[doc = "Register MB5_32B_WORD5 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB5_32B_WORD5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_23`"]
pub type DATA_BYTE_23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_23`"]
pub struct DATA_BYTE_23_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_22`"]
pub type DATA_BYTE_22_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_22`"]
pub struct DATA_BYTE_22_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_21`"]
pub type DATA_BYTE_21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_21`"]
pub struct DATA_BYTE_21_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_20`"]
pub type DATA_BYTE_20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_20`"]
pub struct DATA_BYTE_20_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_20_W<'a> {
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
    pub fn data_byte_23(&self) -> DATA_BYTE_23_R {
        DATA_BYTE_23_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_22(&self) -> DATA_BYTE_22_R {
        DATA_BYTE_22_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_21(&self) -> DATA_BYTE_21_R {
        DATA_BYTE_21_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_20(&self) -> DATA_BYTE_20_R {
        DATA_BYTE_20_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_23(&mut self) -> DATA_BYTE_23_W {
        DATA_BYTE_23_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_22(&mut self) -> DATA_BYTE_22_W {
        DATA_BYTE_22_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_21(&mut self) -> DATA_BYTE_21_W {
        DATA_BYTE_21_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_20(&mut self) -> DATA_BYTE_20_W {
        DATA_BYTE_20_W { w: self }
    }
}

#[doc = "Reader of register MB13_64B_WORD12"]
pub type R = crate::R<u32, super::MB13_64B_WORD12>;
#[doc = "Writer for register MB13_64B_WORD12"]
pub type W = crate::W<u32, super::MB13_64B_WORD12>;
#[doc = "Register MB13_64B_WORD12 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB13_64B_WORD12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_51`"]
pub type DATA_BYTE_51_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_51`"]
pub struct DATA_BYTE_51_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_51_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_50`"]
pub type DATA_BYTE_50_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_50`"]
pub struct DATA_BYTE_50_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_50_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_49`"]
pub type DATA_BYTE_49_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_49`"]
pub struct DATA_BYTE_49_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_49_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_48`"]
pub type DATA_BYTE_48_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_48`"]
pub struct DATA_BYTE_48_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_48_W<'a> {
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
    pub fn data_byte_51(&self) -> DATA_BYTE_51_R {
        DATA_BYTE_51_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_50(&self) -> DATA_BYTE_50_R {
        DATA_BYTE_50_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_49(&self) -> DATA_BYTE_49_R {
        DATA_BYTE_49_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_48(&self) -> DATA_BYTE_48_R {
        DATA_BYTE_48_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_51(&mut self) -> DATA_BYTE_51_W {
        DATA_BYTE_51_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_50(&mut self) -> DATA_BYTE_50_W {
        DATA_BYTE_50_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_49(&mut self) -> DATA_BYTE_49_W {
        DATA_BYTE_49_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_48(&mut self) -> DATA_BYTE_48_W {
        DATA_BYTE_48_W { w: self }
    }
}

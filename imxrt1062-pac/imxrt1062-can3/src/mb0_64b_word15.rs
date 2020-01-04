#[doc = "Reader of register MB0_64B_WORD15"]
pub type R = crate::R<u32, super::MB0_64B_WORD15>;
#[doc = "Writer for register MB0_64B_WORD15"]
pub type W = crate::W<u32, super::MB0_64B_WORD15>;
#[doc = "Register MB0_64B_WORD15 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB0_64B_WORD15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_63`"]
pub type DATA_BYTE_63_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_63`"]
pub struct DATA_BYTE_63_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_63_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_62`"]
pub type DATA_BYTE_62_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_62`"]
pub struct DATA_BYTE_62_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_62_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_61`"]
pub type DATA_BYTE_61_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_61`"]
pub struct DATA_BYTE_61_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_61_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_60`"]
pub type DATA_BYTE_60_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_60`"]
pub struct DATA_BYTE_60_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_60_W<'a> {
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
    pub fn data_byte_63(&self) -> DATA_BYTE_63_R {
        DATA_BYTE_63_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_62(&self) -> DATA_BYTE_62_R {
        DATA_BYTE_62_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_61(&self) -> DATA_BYTE_61_R {
        DATA_BYTE_61_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_60(&self) -> DATA_BYTE_60_R {
        DATA_BYTE_60_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_63(&mut self) -> DATA_BYTE_63_W {
        DATA_BYTE_63_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_62(&mut self) -> DATA_BYTE_62_W {
        DATA_BYTE_62_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_61(&mut self) -> DATA_BYTE_61_W {
        DATA_BYTE_61_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_60(&mut self) -> DATA_BYTE_60_W {
        DATA_BYTE_60_W { w: self }
    }
}

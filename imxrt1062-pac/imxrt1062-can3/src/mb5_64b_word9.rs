#[doc = "Reader of register MB5_64B_WORD9"]
pub type R = crate::R<u32, super::MB5_64B_WORD9>;
#[doc = "Writer for register MB5_64B_WORD9"]
pub type W = crate::W<u32, super::MB5_64B_WORD9>;
#[doc = "Register MB5_64B_WORD9 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB5_64B_WORD9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_39`"]
pub type DATA_BYTE_39_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_39`"]
pub struct DATA_BYTE_39_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_39_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_38`"]
pub type DATA_BYTE_38_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_38`"]
pub struct DATA_BYTE_38_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_38_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_37`"]
pub type DATA_BYTE_37_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_37`"]
pub struct DATA_BYTE_37_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_37_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_36`"]
pub type DATA_BYTE_36_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_36`"]
pub struct DATA_BYTE_36_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_36_W<'a> {
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
    pub fn data_byte_39(&self) -> DATA_BYTE_39_R {
        DATA_BYTE_39_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_38(&self) -> DATA_BYTE_38_R {
        DATA_BYTE_38_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_37(&self) -> DATA_BYTE_37_R {
        DATA_BYTE_37_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_36(&self) -> DATA_BYTE_36_R {
        DATA_BYTE_36_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_39(&mut self) -> DATA_BYTE_39_W {
        DATA_BYTE_39_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_38(&mut self) -> DATA_BYTE_38_W {
        DATA_BYTE_38_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_37(&mut self) -> DATA_BYTE_37_W {
        DATA_BYTE_37_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_36(&mut self) -> DATA_BYTE_36_W {
        DATA_BYTE_36_W { w: self }
    }
}

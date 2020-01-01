#[doc = "Reader of register MB1_64B_WORD14"]
pub type R = crate::R<u32, super::MB1_64B_WORD14>;
#[doc = "Writer for register MB1_64B_WORD14"]
pub type W = crate::W<u32, super::MB1_64B_WORD14>;
#[doc = "Register MB1_64B_WORD14 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB1_64B_WORD14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_59`"]
pub type DATA_BYTE_59_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_59`"]
pub struct DATA_BYTE_59_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_59_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_58`"]
pub type DATA_BYTE_58_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_58`"]
pub struct DATA_BYTE_58_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_58_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_57`"]
pub type DATA_BYTE_57_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_57`"]
pub struct DATA_BYTE_57_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_57_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_56`"]
pub type DATA_BYTE_56_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_56`"]
pub struct DATA_BYTE_56_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_56_W<'a> {
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
    pub fn data_byte_59(&self) -> DATA_BYTE_59_R {
        DATA_BYTE_59_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_58(&self) -> DATA_BYTE_58_R {
        DATA_BYTE_58_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_57(&self) -> DATA_BYTE_57_R {
        DATA_BYTE_57_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_56(&self) -> DATA_BYTE_56_R {
        DATA_BYTE_56_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_59(&mut self) -> DATA_BYTE_59_W {
        DATA_BYTE_59_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_58(&mut self) -> DATA_BYTE_58_W {
        DATA_BYTE_58_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_57(&mut self) -> DATA_BYTE_57_W {
        DATA_BYTE_57_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_56(&mut self) -> DATA_BYTE_56_W {
        DATA_BYTE_56_W { w: self }
    }
}

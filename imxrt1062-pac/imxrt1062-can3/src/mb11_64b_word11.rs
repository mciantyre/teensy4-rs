#[doc = "Reader of register MB11_64B_WORD11"]
pub type R = crate::R<u32, super::MB11_64B_WORD11>;
#[doc = "Writer for register MB11_64B_WORD11"]
pub type W = crate::W<u32, super::MB11_64B_WORD11>;
#[doc = "Register MB11_64B_WORD11 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB11_64B_WORD11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_47`"]
pub type DATA_BYTE_47_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_47`"]
pub struct DATA_BYTE_47_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_47_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_46`"]
pub type DATA_BYTE_46_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_46`"]
pub struct DATA_BYTE_46_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_46_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_45`"]
pub type DATA_BYTE_45_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_45`"]
pub struct DATA_BYTE_45_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_45_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_44`"]
pub type DATA_BYTE_44_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_44`"]
pub struct DATA_BYTE_44_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_44_W<'a> {
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
    pub fn data_byte_47(&self) -> DATA_BYTE_47_R {
        DATA_BYTE_47_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_46(&self) -> DATA_BYTE_46_R {
        DATA_BYTE_46_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_45(&self) -> DATA_BYTE_45_R {
        DATA_BYTE_45_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_44(&self) -> DATA_BYTE_44_R {
        DATA_BYTE_44_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_47(&mut self) -> DATA_BYTE_47_W {
        DATA_BYTE_47_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_46(&mut self) -> DATA_BYTE_46_W {
        DATA_BYTE_46_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_45(&mut self) -> DATA_BYTE_45_W {
        DATA_BYTE_45_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_44(&mut self) -> DATA_BYTE_44_W {
        DATA_BYTE_44_W { w: self }
    }
}

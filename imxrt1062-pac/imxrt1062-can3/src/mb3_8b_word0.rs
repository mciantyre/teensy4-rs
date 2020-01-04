#[doc = "Reader of register MB3_8B_WORD0"]
pub type R = crate::R<u32, super::MB3_8B_WORD0>;
#[doc = "Writer for register MB3_8B_WORD0"]
pub type W = crate::W<u32, super::MB3_8B_WORD0>;
#[doc = "Register MB3_8B_WORD0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MB3_8B_WORD0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE_3`"]
pub type DATA_BYTE_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_3`"]
pub struct DATA_BYTE_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_2`"]
pub type DATA_BYTE_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_2`"]
pub struct DATA_BYTE_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_1`"]
pub type DATA_BYTE_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_1`"]
pub struct DATA_BYTE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA_BYTE_0`"]
pub type DATA_BYTE_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE_0`"]
pub struct DATA_BYTE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_0_W<'a> {
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
    pub fn data_byte_3(&self) -> DATA_BYTE_3_R {
        DATA_BYTE_3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_2(&self) -> DATA_BYTE_2_R {
        DATA_BYTE_2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_1(&self) -> DATA_BYTE_1_R {
        DATA_BYTE_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_0(&self) -> DATA_BYTE_0_R {
        DATA_BYTE_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_3(&mut self) -> DATA_BYTE_3_W {
        DATA_BYTE_3_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_2(&mut self) -> DATA_BYTE_2_W {
        DATA_BYTE_2_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_1(&mut self) -> DATA_BYTE_1_W {
        DATA_BYTE_1_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_0(&mut self) -> DATA_BYTE_0_W {
        DATA_BYTE_0_W { w: self }
    }
}

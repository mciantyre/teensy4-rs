#[doc = "Reader of register TRIG2_CHAIN_3_2"]
pub type R = crate::R<u32, super::TRIG2_CHAIN_3_2>;
#[doc = "Writer for register TRIG2_CHAIN_3_2"]
pub type W = crate::W<u32, super::TRIG2_CHAIN_3_2>;
#[doc = "Register TRIG2_CHAIN_3_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIG2_CHAIN_3_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSEL2`"]
pub type CSEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSEL2`"]
pub struct CSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `HWTS2`"]
pub type HWTS2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HWTS2`"]
pub struct HWTS2_W<'a> {
    w: &'a mut W,
}
impl<'a> HWTS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
        self.w
    }
}
#[doc = "Reader of field `B2B2`"]
pub type B2B2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2B2`"]
pub struct B2B2_W<'a> {
    w: &'a mut W,
}
impl<'a> B2B2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `IE2`"]
pub type IE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IE2`"]
pub struct IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> IE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `CSEL3`"]
pub type CSEL3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSEL3`"]
pub struct CSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `HWTS3`"]
pub type HWTS3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HWTS3`"]
pub struct HWTS3_W<'a> {
    w: &'a mut W,
}
impl<'a> HWTS3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Reader of field `B2B3`"]
pub type B2B3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2B3`"]
pub struct B2B3_W<'a> {
    w: &'a mut W,
}
impl<'a> B2B3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `IE3`"]
pub type IE3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IE3`"]
pub struct IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> IE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CHAIN2 CSEL"]
    #[inline(always)]
    pub fn csel2(&self) -> CSEL2_R {
        CSEL2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - CHAIN2 HWTS"]
    #[inline(always)]
    pub fn hwts2(&self) -> HWTS2_R {
        HWTS2_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - CHAIN2 B2B"]
    #[inline(always)]
    pub fn b2b2(&self) -> B2B2_R {
        B2B2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - CHAIN2 IE"]
    #[inline(always)]
    pub fn ie2(&self) -> IE2_R {
        IE2_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - CHAIN3 CSEL"]
    #[inline(always)]
    pub fn csel3(&self) -> CSEL3_R {
        CSEL3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - CHAIN3 HWTS"]
    #[inline(always)]
    pub fn hwts3(&self) -> HWTS3_R {
        HWTS3_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - CHAIN3 B2B"]
    #[inline(always)]
    pub fn b2b3(&self) -> B2B3_R {
        B2B3_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - CHAIN3 IE"]
    #[inline(always)]
    pub fn ie3(&self) -> IE3_R {
        IE3_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CHAIN2 CSEL"]
    #[inline(always)]
    pub fn csel2(&mut self) -> CSEL2_W {
        CSEL2_W { w: self }
    }
    #[doc = "Bits 4:11 - CHAIN2 HWTS"]
    #[inline(always)]
    pub fn hwts2(&mut self) -> HWTS2_W {
        HWTS2_W { w: self }
    }
    #[doc = "Bit 12 - CHAIN2 B2B"]
    #[inline(always)]
    pub fn b2b2(&mut self) -> B2B2_W {
        B2B2_W { w: self }
    }
    #[doc = "Bits 13:14 - CHAIN2 IE"]
    #[inline(always)]
    pub fn ie2(&mut self) -> IE2_W {
        IE2_W { w: self }
    }
    #[doc = "Bits 16:19 - CHAIN3 CSEL"]
    #[inline(always)]
    pub fn csel3(&mut self) -> CSEL3_W {
        CSEL3_W { w: self }
    }
    #[doc = "Bits 20:27 - CHAIN3 HWTS"]
    #[inline(always)]
    pub fn hwts3(&mut self) -> HWTS3_W {
        HWTS3_W { w: self }
    }
    #[doc = "Bit 28 - CHAIN3 B2B"]
    #[inline(always)]
    pub fn b2b3(&mut self) -> B2B3_W {
        B2B3_W { w: self }
    }
    #[doc = "Bits 29:30 - CHAIN3 IE"]
    #[inline(always)]
    pub fn ie3(&mut self) -> IE3_W {
        IE3_W { w: self }
    }
}

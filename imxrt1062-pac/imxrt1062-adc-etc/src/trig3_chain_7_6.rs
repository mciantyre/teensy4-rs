#[doc = "Reader of register TRIG3_CHAIN_7_6"]
pub type R = crate::R<u32, super::TRIG3_CHAIN_7_6>;
#[doc = "Writer for register TRIG3_CHAIN_7_6"]
pub type W = crate::W<u32, super::TRIG3_CHAIN_7_6>;
#[doc = "Register TRIG3_CHAIN_7_6 `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIG3_CHAIN_7_6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSEL6`"]
pub type CSEL6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSEL6`"]
pub struct CSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `HWTS6`"]
pub type HWTS6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HWTS6`"]
pub struct HWTS6_W<'a> {
    w: &'a mut W,
}
impl<'a> HWTS6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
        self.w
    }
}
#[doc = "Reader of field `B2B6`"]
pub type B2B6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2B6`"]
pub struct B2B6_W<'a> {
    w: &'a mut W,
}
impl<'a> B2B6_W<'a> {
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
#[doc = "Reader of field `IE6`"]
pub type IE6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IE6`"]
pub struct IE6_W<'a> {
    w: &'a mut W,
}
impl<'a> IE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `CSEL7`"]
pub type CSEL7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSEL7`"]
pub struct CSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `HWTS7`"]
pub type HWTS7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HWTS7`"]
pub struct HWTS7_W<'a> {
    w: &'a mut W,
}
impl<'a> HWTS7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Reader of field `B2B7`"]
pub type B2B7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2B7`"]
pub struct B2B7_W<'a> {
    w: &'a mut W,
}
impl<'a> B2B7_W<'a> {
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
#[doc = "Reader of field `IE7`"]
pub type IE7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IE7`"]
pub struct IE7_W<'a> {
    w: &'a mut W,
}
impl<'a> IE7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CHAIN6 CSEL"]
    #[inline(always)]
    pub fn csel6(&self) -> CSEL6_R {
        CSEL6_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - CHAIN6 HWTS"]
    #[inline(always)]
    pub fn hwts6(&self) -> HWTS6_R {
        HWTS6_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - CHAIN6 B2B"]
    #[inline(always)]
    pub fn b2b6(&self) -> B2B6_R {
        B2B6_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - CHAIN6 IE"]
    #[inline(always)]
    pub fn ie6(&self) -> IE6_R {
        IE6_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - CHAIN7 CSEL"]
    #[inline(always)]
    pub fn csel7(&self) -> CSEL7_R {
        CSEL7_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - CHAIN7 HWTS"]
    #[inline(always)]
    pub fn hwts7(&self) -> HWTS7_R {
        HWTS7_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - CHAIN7 B2B"]
    #[inline(always)]
    pub fn b2b7(&self) -> B2B7_R {
        B2B7_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - CHAIN7 IE"]
    #[inline(always)]
    pub fn ie7(&self) -> IE7_R {
        IE7_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CHAIN6 CSEL"]
    #[inline(always)]
    pub fn csel6(&mut self) -> CSEL6_W {
        CSEL6_W { w: self }
    }
    #[doc = "Bits 4:11 - CHAIN6 HWTS"]
    #[inline(always)]
    pub fn hwts6(&mut self) -> HWTS6_W {
        HWTS6_W { w: self }
    }
    #[doc = "Bit 12 - CHAIN6 B2B"]
    #[inline(always)]
    pub fn b2b6(&mut self) -> B2B6_W {
        B2B6_W { w: self }
    }
    #[doc = "Bits 13:14 - CHAIN6 IE"]
    #[inline(always)]
    pub fn ie6(&mut self) -> IE6_W {
        IE6_W { w: self }
    }
    #[doc = "Bits 16:19 - CHAIN7 CSEL"]
    #[inline(always)]
    pub fn csel7(&mut self) -> CSEL7_W {
        CSEL7_W { w: self }
    }
    #[doc = "Bits 20:27 - CHAIN7 HWTS"]
    #[inline(always)]
    pub fn hwts7(&mut self) -> HWTS7_W {
        HWTS7_W { w: self }
    }
    #[doc = "Bit 28 - CHAIN7 B2B"]
    #[inline(always)]
    pub fn b2b7(&mut self) -> B2B7_W {
        B2B7_W { w: self }
    }
    #[doc = "Bits 29:30 - CHAIN7 IE"]
    #[inline(always)]
    pub fn ie7(&mut self) -> IE7_W {
        IE7_W { w: self }
    }
}

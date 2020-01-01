#[doc = "Reader of register TRIG0_CHAIN_5_4"]
pub type R = crate::R<u32, super::TRIG0_CHAIN_5_4>;
#[doc = "Writer for register TRIG0_CHAIN_5_4"]
pub type W = crate::W<u32, super::TRIG0_CHAIN_5_4>;
#[doc = "Register TRIG0_CHAIN_5_4 `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIG0_CHAIN_5_4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSEL4`"]
pub type CSEL4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSEL4`"]
pub struct CSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `HWTS4`"]
pub type HWTS4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HWTS4`"]
pub struct HWTS4_W<'a> {
    w: &'a mut W,
}
impl<'a> HWTS4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
        self.w
    }
}
#[doc = "Reader of field `B2B4`"]
pub type B2B4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2B4`"]
pub struct B2B4_W<'a> {
    w: &'a mut W,
}
impl<'a> B2B4_W<'a> {
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
#[doc = "Reader of field `IE4`"]
pub type IE4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IE4`"]
pub struct IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> IE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `CSEL5`"]
pub type CSEL5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSEL5`"]
pub struct CSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `HWTS5`"]
pub type HWTS5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HWTS5`"]
pub struct HWTS5_W<'a> {
    w: &'a mut W,
}
impl<'a> HWTS5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Reader of field `B2B5`"]
pub type B2B5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2B5`"]
pub struct B2B5_W<'a> {
    w: &'a mut W,
}
impl<'a> B2B5_W<'a> {
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
#[doc = "Reader of field `IE5`"]
pub type IE5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IE5`"]
pub struct IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> IE5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CHAIN4 CSEL"]
    #[inline(always)]
    pub fn csel4(&self) -> CSEL4_R {
        CSEL4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - CHAIN4 HWTS"]
    #[inline(always)]
    pub fn hwts4(&self) -> HWTS4_R {
        HWTS4_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - CHAIN4 B2B"]
    #[inline(always)]
    pub fn b2b4(&self) -> B2B4_R {
        B2B4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - CHAIN4 IE"]
    #[inline(always)]
    pub fn ie4(&self) -> IE4_R {
        IE4_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - CHAIN5 CSEL"]
    #[inline(always)]
    pub fn csel5(&self) -> CSEL5_R {
        CSEL5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - CHAIN5 HWTS"]
    #[inline(always)]
    pub fn hwts5(&self) -> HWTS5_R {
        HWTS5_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - CHAIN5 B2B"]
    #[inline(always)]
    pub fn b2b5(&self) -> B2B5_R {
        B2B5_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - CHAIN5 IE"]
    #[inline(always)]
    pub fn ie5(&self) -> IE5_R {
        IE5_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CHAIN4 CSEL"]
    #[inline(always)]
    pub fn csel4(&mut self) -> CSEL4_W {
        CSEL4_W { w: self }
    }
    #[doc = "Bits 4:11 - CHAIN4 HWTS"]
    #[inline(always)]
    pub fn hwts4(&mut self) -> HWTS4_W {
        HWTS4_W { w: self }
    }
    #[doc = "Bit 12 - CHAIN4 B2B"]
    #[inline(always)]
    pub fn b2b4(&mut self) -> B2B4_W {
        B2B4_W { w: self }
    }
    #[doc = "Bits 13:14 - CHAIN4 IE"]
    #[inline(always)]
    pub fn ie4(&mut self) -> IE4_W {
        IE4_W { w: self }
    }
    #[doc = "Bits 16:19 - CHAIN5 CSEL"]
    #[inline(always)]
    pub fn csel5(&mut self) -> CSEL5_W {
        CSEL5_W { w: self }
    }
    #[doc = "Bits 20:27 - CHAIN5 HWTS"]
    #[inline(always)]
    pub fn hwts5(&mut self) -> HWTS5_W {
        HWTS5_W { w: self }
    }
    #[doc = "Bit 28 - CHAIN5 B2B"]
    #[inline(always)]
    pub fn b2b5(&mut self) -> B2B5_W {
        B2B5_W { w: self }
    }
    #[doc = "Bits 29:30 - CHAIN5 IE"]
    #[inline(always)]
    pub fn ie5(&mut self) -> IE5_W {
        IE5_W { w: self }
    }
}

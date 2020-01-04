#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u8, u8>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR`"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RELOAD_SHADOWS`"]
pub type RELOAD_SHADOWS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RELOAD_SHADOWS`"]
pub struct RELOAD_SHADOWS_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_SHADOWS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CRC_TEST`"]
pub type CRC_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC_TEST`"]
pub struct CRC_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_TEST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CRC_FAIL`"]
pub type CRC_FAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC_FAIL`"]
pub struct CRC_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_FAIL_W<'a> {
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
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u8, u8>;
#[doc = "WR_UNLOCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum WR_UNLOCK_A {
    #[doc = "15991: Key needed to unlock HW_OCOTP_DATA register."]
    KEY = 15991,
}
impl From<WR_UNLOCK_A> for u16 {
    #[inline(always)]
    fn from(variant: WR_UNLOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WR_UNLOCK`"]
pub type WR_UNLOCK_R = crate::R<u16, WR_UNLOCK_A>;
impl WR_UNLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, WR_UNLOCK_A> {
        use crate::Variant::*;
        match self.bits {
            15991 => Val(WR_UNLOCK_A::KEY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == WR_UNLOCK_A::KEY
    }
}
#[doc = "Write proxy for field `WR_UNLOCK`"]
pub struct WR_UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_UNLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WR_UNLOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Key needed to unlock HW_OCOTP_DATA register."]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(WR_UNLOCK_A::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - RSVD0"]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RELOAD_SHADOWS"]
    #[inline(always)]
    pub fn reload_shadows(&self) -> RELOAD_SHADOWS_R {
        RELOAD_SHADOWS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CRC_TEST"]
    #[inline(always)]
    pub fn crc_test(&self) -> CRC_TEST_R {
        CRC_TEST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CRC_FAIL"]
    #[inline(always)]
    pub fn crc_fail(&self) -> CRC_FAIL_R {
        CRC_FAIL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - RSVD1"]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:31 - WR_UNLOCK"]
    #[inline(always)]
    pub fn wr_unlock(&self) -> WR_UNLOCK_R {
        WR_UNLOCK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - ADDR"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 9 - ERROR"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Bit 10 - RELOAD_SHADOWS"]
    #[inline(always)]
    pub fn reload_shadows(&mut self) -> RELOAD_SHADOWS_W {
        RELOAD_SHADOWS_W { w: self }
    }
    #[doc = "Bit 11 - CRC_TEST"]
    #[inline(always)]
    pub fn crc_test(&mut self) -> CRC_TEST_W {
        CRC_TEST_W { w: self }
    }
    #[doc = "Bit 12 - CRC_FAIL"]
    #[inline(always)]
    pub fn crc_fail(&mut self) -> CRC_FAIL_W {
        CRC_FAIL_W { w: self }
    }
    #[doc = "Bits 16:31 - WR_UNLOCK"]
    #[inline(always)]
    pub fn wr_unlock(&mut self) -> WR_UNLOCK_W {
        WR_UNLOCK_W { w: self }
    }
}

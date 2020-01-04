#[doc = "Reader of register CRC_ADDR"]
pub type R = crate::R<u32, super::CRC_ADDR>;
#[doc = "Writer for register CRC_ADDR"]
pub type W = crate::W<u32, super::CRC_ADDR>;
#[doc = "Register CRC_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_START_ADDR`"]
pub type DATA_START_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_START_ADDR`"]
pub struct DATA_START_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_START_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_END_ADDR`"]
pub type DATA_END_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_END_ADDR`"]
pub struct DATA_END_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_END_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CRC_ADDR`"]
pub type CRC_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CRC_ADDR`"]
pub struct CRC_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `OTPMK_CRC`"]
pub type OTPMK_CRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTPMK_CRC`"]
pub struct OTPMK_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPMK_CRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - DATA_START_ADDR"]
    #[inline(always)]
    pub fn data_start_addr(&self) -> DATA_START_ADDR_R {
        DATA_START_ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA_END_ADDR"]
    #[inline(always)]
    pub fn data_end_addr(&self) -> DATA_END_ADDR_R {
        DATA_END_ADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CRC_ADDR"]
    #[inline(always)]
    pub fn crc_addr(&self) -> CRC_ADDR_R {
        CRC_ADDR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - OTPMK_CRC"]
    #[inline(always)]
    pub fn otpmk_crc(&self) -> OTPMK_CRC_R {
        OTPMK_CRC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:31 - RSVD0"]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA_START_ADDR"]
    #[inline(always)]
    pub fn data_start_addr(&mut self) -> DATA_START_ADDR_W {
        DATA_START_ADDR_W { w: self }
    }
    #[doc = "Bits 8:15 - DATA_END_ADDR"]
    #[inline(always)]
    pub fn data_end_addr(&mut self) -> DATA_END_ADDR_W {
        DATA_END_ADDR_W { w: self }
    }
    #[doc = "Bits 16:23 - CRC_ADDR"]
    #[inline(always)]
    pub fn crc_addr(&mut self) -> CRC_ADDR_W {
        CRC_ADDR_W { w: self }
    }
    #[doc = "Bit 24 - OTPMK_CRC"]
    #[inline(always)]
    pub fn otpmk_crc(&mut self) -> OTPMK_CRC_W {
        OTPMK_CRC_W { w: self }
    }
}

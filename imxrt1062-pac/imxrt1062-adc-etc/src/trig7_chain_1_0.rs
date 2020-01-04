#[doc = "Reader of register TRIG7_CHAIN_1_0"]
pub type R = crate::R<u32, super::TRIG7_CHAIN_1_0>;
#[doc = "Writer for register TRIG7_CHAIN_1_0"]
pub type W = crate::W<u32, super::TRIG7_CHAIN_1_0>;
#[doc = "Register TRIG7_CHAIN_1_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIG7_CHAIN_1_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSEL0`"]
pub type CSEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSEL0`"]
pub struct CSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `HWTS0`"]
pub type HWTS0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HWTS0`"]
pub struct HWTS0_W<'a> {
    w: &'a mut W,
}
impl<'a> HWTS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
        self.w
    }
}
#[doc = "Reader of field `B2B0`"]
pub type B2B0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2B0`"]
pub struct B2B0_W<'a> {
    w: &'a mut W,
}
impl<'a> B2B0_W<'a> {
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
#[doc = "Reader of field `IE0`"]
pub type IE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IE0`"]
pub struct IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> IE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `CSEL1`"]
pub type CSEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSEL1`"]
pub struct CSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `HWTS1`"]
pub type HWTS1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HWTS1`"]
pub struct HWTS1_W<'a> {
    w: &'a mut W,
}
impl<'a> HWTS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Reader of field `B2B1`"]
pub type B2B1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2B1`"]
pub struct B2B1_W<'a> {
    w: &'a mut W,
}
impl<'a> B2B1_W<'a> {
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
#[doc = "Reader of field `IE1`"]
pub type IE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IE1`"]
pub struct IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> IE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CHAIN0 CSEL ADC channel selection"]
    #[inline(always)]
    pub fn csel0(&self) -> CSEL0_R {
        CSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - CHAIN0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub fn hwts0(&self) -> HWTS0_R {
        HWTS0_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - CHAIN0 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    #[inline(always)]
    pub fn b2b0(&self) -> B2B0_R {
        B2B0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - CHAIN0 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    #[inline(always)]
    pub fn ie0(&self) -> IE0_R {
        IE0_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - CHAIN1 CSEL ADC channel selection"]
    #[inline(always)]
    pub fn csel1(&self) -> CSEL1_R {
        CSEL1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - CHAIN1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub fn hwts1(&self) -> HWTS1_R {
        HWTS1_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - CHAIN1 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    #[inline(always)]
    pub fn b2b1(&self) -> B2B1_R {
        B2B1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - CHAIN1 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    #[inline(always)]
    pub fn ie1(&self) -> IE1_R {
        IE1_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CHAIN0 CSEL ADC channel selection"]
    #[inline(always)]
    pub fn csel0(&mut self) -> CSEL0_W {
        CSEL0_W { w: self }
    }
    #[doc = "Bits 4:11 - CHAIN0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub fn hwts0(&mut self) -> HWTS0_W {
        HWTS0_W { w: self }
    }
    #[doc = "Bit 12 - CHAIN0 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    #[inline(always)]
    pub fn b2b0(&mut self) -> B2B0_W {
        B2B0_W { w: self }
    }
    #[doc = "Bits 13:14 - CHAIN0 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    #[inline(always)]
    pub fn ie0(&mut self) -> IE0_W {
        IE0_W { w: self }
    }
    #[doc = "Bits 16:19 - CHAIN1 CSEL ADC channel selection"]
    #[inline(always)]
    pub fn csel1(&mut self) -> CSEL1_W {
        CSEL1_W { w: self }
    }
    #[doc = "Bits 20:27 - CHAIN1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub fn hwts1(&mut self) -> HWTS1_W {
        HWTS1_W { w: self }
    }
    #[doc = "Bit 28 - CHAIN1 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    #[inline(always)]
    pub fn b2b1(&mut self) -> B2B1_W {
        B2B1_W { w: self }
    }
    #[doc = "Bits 29:30 - CHAIN1 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    #[inline(always)]
    pub fn ie1(&mut self) -> IE1_W {
        IE1_W { w: self }
    }
}

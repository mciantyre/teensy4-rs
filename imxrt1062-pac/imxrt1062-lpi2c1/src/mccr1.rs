#[doc = "Reader of register MCCR1"]
pub type R = crate::R<u32, super::MCCR1>;
#[doc = "Writer for register MCCR1"]
pub type W = crate::W<u32, super::MCCR1>;
#[doc = "Register MCCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MCCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKLO`"]
pub type CLKLO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKLO`"]
pub struct CLKLO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `CLKHI`"]
pub type CLKHI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKHI`"]
pub struct CLKHI_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SETHOLD`"]
pub type SETHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SETHOLD`"]
pub struct SETHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SETHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATAVD`"]
pub type DATAVD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAVD`"]
pub struct DATAVD_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Clock Low Period"]
    #[inline(always)]
    pub fn clklo(&self) -> CLKLO_R {
        CLKLO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Clock High Period"]
    #[inline(always)]
    pub fn clkhi(&self) -> CLKHI_R {
        CLKHI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Setup Hold Delay"]
    #[inline(always)]
    pub fn sethold(&self) -> SETHOLD_R {
        SETHOLD_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Data Valid Delay"]
    #[inline(always)]
    pub fn datavd(&self) -> DATAVD_R {
        DATAVD_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clock Low Period"]
    #[inline(always)]
    pub fn clklo(&mut self) -> CLKLO_W {
        CLKLO_W { w: self }
    }
    #[doc = "Bits 8:13 - Clock High Period"]
    #[inline(always)]
    pub fn clkhi(&mut self) -> CLKHI_W {
        CLKHI_W { w: self }
    }
    #[doc = "Bits 16:21 - Setup Hold Delay"]
    #[inline(always)]
    pub fn sethold(&mut self) -> SETHOLD_W {
        SETHOLD_W { w: self }
    }
    #[doc = "Bits 24:29 - Data Valid Delay"]
    #[inline(always)]
    pub fn datavd(&mut self) -> DATAVD_W {
        DATAVD_W { w: self }
    }
}

#[doc = "Reader of register SCFGR2"]
pub type R = crate::R<u32, super::SCFGR2>;
#[doc = "Writer for register SCFGR2"]
pub type W = crate::W<u32, super::SCFGR2>;
#[doc = "Register SCFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKHOLD`"]
pub type CLKHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKHOLD`"]
pub struct CLKHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
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
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FILTSCL`"]
pub type FILTSCL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILTSCL`"]
pub struct FILTSCL_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTSCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `FILTSDA`"]
pub type FILTSDA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILTSDA`"]
pub struct FILTSDA_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTSDA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock Hold Time"]
    #[inline(always)]
    pub fn clkhold(&self) -> CLKHOLD_R {
        CLKHOLD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Data Valid Delay"]
    #[inline(always)]
    pub fn datavd(&self) -> DATAVD_R {
        DATAVD_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - Glitch Filter SCL"]
    #[inline(always)]
    pub fn filtscl(&self) -> FILTSCL_R {
        FILTSCL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Glitch Filter SDA"]
    #[inline(always)]
    pub fn filtsda(&self) -> FILTSDA_R {
        FILTSDA_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Hold Time"]
    #[inline(always)]
    pub fn clkhold(&mut self) -> CLKHOLD_W {
        CLKHOLD_W { w: self }
    }
    #[doc = "Bits 8:13 - Data Valid Delay"]
    #[inline(always)]
    pub fn datavd(&mut self) -> DATAVD_W {
        DATAVD_W { w: self }
    }
    #[doc = "Bits 16:19 - Glitch Filter SCL"]
    #[inline(always)]
    pub fn filtscl(&mut self) -> FILTSCL_W {
        FILTSCL_W { w: self }
    }
    #[doc = "Bits 24:27 - Glitch Filter SDA"]
    #[inline(always)]
    pub fn filtsda(&mut self) -> FILTSDA_W {
        FILTSDA_W { w: self }
    }
}

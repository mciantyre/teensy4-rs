#[doc = "Reader of register MCFGR2"]
pub type R = crate::R<u32, super::MCFGR2>;
#[doc = "Writer for register MCFGR2"]
pub type W = crate::W<u32, super::MCFGR2>;
#[doc = "Register MCFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MCFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUSIDLE`"]
pub type BUSIDLE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BUSIDLE`"]
pub struct BUSIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSIDLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
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
    #[doc = "Bits 0:11 - Bus Idle Timeout"]
    #[inline(always)]
    pub fn busidle(&self) -> BUSIDLE_R {
        BUSIDLE_R::new((self.bits & 0x0fff) as u16)
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
    #[doc = "Bits 0:11 - Bus Idle Timeout"]
    #[inline(always)]
    pub fn busidle(&mut self) -> BUSIDLE_W {
        BUSIDLE_W { w: self }
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

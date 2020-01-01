#[doc = "Reader of register CNT"]
pub type R = crate::R<u32, super::CNT>;
#[doc = "Writer for register CNT"]
pub type W = crate::W<u32, super::CNT>;
#[doc = "Register CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNTLOW`"]
pub type CNTLOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNTLOW`"]
pub struct CNTLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CNTHIGH`"]
pub type CNTHIGH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNTHIGH`"]
pub struct CNTHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cntlow(&self) -> CNTLOW_R {
        CNTLOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cnthigh(&self) -> CNTHIGH_R {
        CNTHIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cntlow(&mut self) -> CNTLOW_W {
        CNTLOW_W { w: self }
    }
    #[doc = "Bits 8:15 - High byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cnthigh(&mut self) -> CNTHIGH_W {
        CNTHIGH_W { w: self }
    }
}

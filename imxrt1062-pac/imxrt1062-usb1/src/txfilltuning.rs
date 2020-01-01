#[doc = "Reader of register TXFILLTUNING"]
pub type R = crate::R<u32, super::TXFILLTUNING>;
#[doc = "Writer for register TXFILLTUNING"]
pub type W = crate::W<u32, super::TXFILLTUNING>;
#[doc = "Register TXFILLTUNING `reset()`'s with value 0"]
impl crate::ResetValue for super::TXFILLTUNING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXSCHOH`"]
pub type TXSCHOH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXSCHOH`"]
pub struct TXSCHOH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSCHOH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TXSCHHEALTH`"]
pub type TXSCHHEALTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXSCHHEALTH`"]
pub struct TXSCHHEALTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSCHHEALTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TXFIFOTHRES`"]
pub type TXFIFOTHRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXFIFOTHRES`"]
pub struct TXFIFOTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Scheduler Overhead"]
    #[inline(always)]
    pub fn txschoh(&self) -> TXSCHOH_R {
        TXSCHOH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Scheduler Health Counter"]
    #[inline(always)]
    pub fn txschhealth(&self) -> TXSCHHEALTH_R {
        TXSCHHEALTH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:21 - FIFO Burst Threshold"]
    #[inline(always)]
    pub fn txfifothres(&self) -> TXFIFOTHRES_R {
        TXFIFOTHRES_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Scheduler Overhead"]
    #[inline(always)]
    pub fn txschoh(&mut self) -> TXSCHOH_W {
        TXSCHOH_W { w: self }
    }
    #[doc = "Bits 8:12 - Scheduler Health Counter"]
    #[inline(always)]
    pub fn txschhealth(&mut self) -> TXSCHHEALTH_W {
        TXSCHHEALTH_W { w: self }
    }
    #[doc = "Bits 16:21 - FIFO Burst Threshold"]
    #[inline(always)]
    pub fn txfifothres(&mut self) -> TXFIFOTHRES_W {
        TXFIFOTHRES_W { w: self }
    }
}

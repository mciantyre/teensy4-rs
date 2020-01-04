#[doc = "Reader of register WIN"]
pub type R = crate::R<u32, super::WIN>;
#[doc = "Writer for register WIN"]
pub type W = crate::W<u32, super::WIN>;
#[doc = "Register WIN `reset()`'s with value 0"]
impl crate::ResetValue for super::WIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WINLOW`"]
pub type WINLOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WINLOW`"]
pub struct WINLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> WINLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WINHIGH`"]
pub type WINHIGH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WINHIGH`"]
pub struct WINHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> WINHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low byte of Watchdog Window"]
    #[inline(always)]
    pub fn winlow(&self) -> WINLOW_R {
        WINLOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High byte of Watchdog Window"]
    #[inline(always)]
    pub fn winhigh(&self) -> WINHIGH_R {
        WINHIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low byte of Watchdog Window"]
    #[inline(always)]
    pub fn winlow(&mut self) -> WINLOW_W {
        WINLOW_W { w: self }
    }
    #[doc = "Bits 8:15 - High byte of Watchdog Window"]
    #[inline(always)]
    pub fn winhigh(&mut self) -> WINHIGH_W {
        WINHIGH_W { w: self }
    }
}

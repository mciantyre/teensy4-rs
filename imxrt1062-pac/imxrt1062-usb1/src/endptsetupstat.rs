#[doc = "Reader of register ENDPTSETUPSTAT"]
pub type R = crate::R<u32, super::ENDPTSETUPSTAT>;
#[doc = "Writer for register ENDPTSETUPSTAT"]
pub type W = crate::W<u32, super::ENDPTSETUPSTAT>;
#[doc = "Register ENDPTSETUPSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::ENDPTSETUPSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENDPTSETUPSTAT`"]
pub type ENDPTSETUPSTAT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ENDPTSETUPSTAT`"]
pub struct ENDPTSETUPSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDPTSETUPSTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Setup Endpoint Status"]
    #[inline(always)]
    pub fn endptsetupstat(&self) -> ENDPTSETUPSTAT_R {
        ENDPTSETUPSTAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Setup Endpoint Status"]
    #[inline(always)]
    pub fn endptsetupstat(&mut self) -> ENDPTSETUPSTAT_W {
        ENDPTSETUPSTAT_W { w: self }
    }
}

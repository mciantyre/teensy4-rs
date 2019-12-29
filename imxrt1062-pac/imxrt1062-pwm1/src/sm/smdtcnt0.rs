#[doc = "Reader of register SMDTCNT0"]
pub type R = crate::R<u16, super::SMDTCNT0>;
#[doc = "Writer for register SMDTCNT0"]
pub type W = crate::W<u16, super::SMDTCNT0>;
#[doc = "Register SMDTCNT0 `reset()`'s with value 0x07ff"]
impl crate::ResetValue for super::SMDTCNT0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07ff
    }
}
#[doc = "Reader of field `DTCNT0`"]
pub type DTCNT0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DTCNT0`"]
pub struct DTCNT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCNT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DTCNT0"]
    #[inline(always)]
    pub fn dtcnt0(&self) -> DTCNT0_R {
        DTCNT0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DTCNT0"]
    #[inline(always)]
    pub fn dtcnt0(&mut self) -> DTCNT0_W {
        DTCNT0_W { w: self }
    }
}

#[doc = "Reader of register SM3DTCNT1"]
pub type R = crate::R<u16, super::SM3DTCNT1>;
#[doc = "Writer for register SM3DTCNT1"]
pub type W = crate::W<u16, super::SM3DTCNT1>;
#[doc = "Register SM3DTCNT1 `reset()`'s with value 0x07ff"]
impl crate::ResetValue for super::SM3DTCNT1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07ff
    }
}
#[doc = "Reader of field `DTCNT1`"]
pub type DTCNT1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DTCNT1`"]
pub struct DTCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCNT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DTCNT1"]
    #[inline(always)]
    pub fn dtcnt1(&self) -> DTCNT1_R {
        DTCNT1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DTCNT1"]
    #[inline(always)]
    pub fn dtcnt1(&mut self) -> DTCNT1_W {
        DTCNT1_W { w: self }
    }
}

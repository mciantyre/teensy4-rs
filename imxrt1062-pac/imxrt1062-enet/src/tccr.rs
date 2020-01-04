#[doc = "Reader of register TCCR%s"]
pub type R = crate::R<u32, super::TCCR>;
#[doc = "Writer for register TCCR%s"]
pub type W = crate::W<u32, super::TCCR>;
#[doc = "Register TCCR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::TCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCC`"]
pub type TCC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TCC`"]
pub struct TCC_W<'a> {
    w: &'a mut W,
}
impl<'a> TCC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer Capture Compare"]
    #[inline(always)]
    pub fn tcc(&self) -> TCC_R {
        TCC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer Capture Compare"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TCC_W {
        TCC_W { w: self }
    }
}

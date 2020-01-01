#[doc = "Reader of register LPPGDR"]
pub type R = crate::R<u32, super::LPPGDR>;
#[doc = "Writer for register LPPGDR"]
pub type W = crate::W<u32, super::LPPGDR>;
#[doc = "Register LPPGDR `reset()`'s with value 0"]
impl crate::ResetValue for super::LPPGDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PGD`"]
pub type PGD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PGD`"]
pub struct PGD_W<'a> {
    w: &'a mut W,
}
impl<'a> PGD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Power Glitch Detector Value"]
    #[inline(always)]
    pub fn pgd(&self) -> PGD_R {
        PGD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Power Glitch Detector Value"]
    #[inline(always)]
    pub fn pgd(&mut self) -> PGD_W {
        PGD_W { w: self }
    }
}

#[doc = "Writer for register ICIMVAU"]
pub type W = crate::W<u32, super::ICIMVAU>;
#[doc = "Register ICIMVAU `reset()`'s with value 0"]
impl crate::ResetValue for super::ICIMVAU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ICIMVAU`"]
pub struct ICIMVAU_W<'a> {
    w: &'a mut W,
}
impl<'a> ICIMVAU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - I-cache invalidate by MVA to PoU"]
    #[inline(always)]
    pub fn icimvau(&mut self) -> ICIMVAU_W {
        ICIMVAU_W { w: self }
    }
}

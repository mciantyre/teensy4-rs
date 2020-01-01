#[doc = "Writer for register DCCMVAU"]
pub type W = crate::W<u32, super::DCCMVAU>;
#[doc = "Register DCCMVAU `reset()`'s with value 0"]
impl crate::ResetValue for super::DCCMVAU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DCCMVAU`"]
pub struct DCCMVAU_W<'a> {
    w: &'a mut W,
}
impl<'a> DCCMVAU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - D-cache clean by MVA to PoU"]
    #[inline(always)]
    pub fn dccmvau(&mut self) -> DCCMVAU_W {
        DCCMVAU_W { w: self }
    }
}

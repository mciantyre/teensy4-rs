#[doc = "Writer for register DCCMVAC"]
pub type W = crate::W<u32, super::DCCMVAC>;
#[doc = "Register DCCMVAC `reset()`'s with value 0"]
impl crate::ResetValue for super::DCCMVAC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DCCMVAC`"]
pub struct DCCMVAC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCCMVAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - D-cache clean by MVA to PoC"]
    #[inline(always)]
    pub fn dccmvac(&mut self) -> DCCMVAC_W {
        DCCMVAC_W { w: self }
    }
}

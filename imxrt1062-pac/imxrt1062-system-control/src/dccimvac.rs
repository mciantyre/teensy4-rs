#[doc = "Writer for register DCCIMVAC"]
pub type W = crate::W<u32, super::DCCIMVAC>;
#[doc = "Register DCCIMVAC `reset()`'s with value 0"]
impl crate::ResetValue for super::DCCIMVAC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DCCIMVAC`"]
pub struct DCCIMVAC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCCIMVAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - D-cache clean and invalidate by MVA to PoC"]
    #[inline(always)]
    pub fn dccimvac(&mut self) -> DCCIMVAC_W {
        DCCIMVAC_W { w: self }
    }
}

#[doc = "Writer for register DCIMVAC"]
pub type W = crate::W<u32, super::DCIMVAC>;
#[doc = "Register DCIMVAC `reset()`'s with value 0"]
impl crate::ResetValue for super::DCIMVAC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DCIMVAC`"]
pub struct DCIMVAC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIMVAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - D-cache invalidate by MVA to PoC"]
    #[inline(always)]
    pub fn dcimvac(&mut self) -> DCIMVAC_W {
        DCIMVAC_W { w: self }
    }
}

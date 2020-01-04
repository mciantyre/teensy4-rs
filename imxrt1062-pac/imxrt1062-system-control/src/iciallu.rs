#[doc = "Writer for register ICIALLU"]
pub type W = crate::W<u32, super::ICIALLU>;
#[doc = "Register ICIALLU `reset()`'s with value 0"]
impl crate::ResetValue for super::ICIALLU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ICIALLU`"]
pub struct ICIALLU_W<'a> {
    w: &'a mut W,
}
impl<'a> ICIALLU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - I-cache invalidate all to PoU"]
    #[inline(always)]
    pub fn iciallu(&mut self) -> ICIALLU_W {
        ICIALLU_W { w: self }
    }
}

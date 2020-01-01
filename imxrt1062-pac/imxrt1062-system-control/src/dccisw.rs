#[doc = "Writer for register DCCISW"]
pub type W = crate::W<u32, super::DCCISW>;
#[doc = "Register DCCISW `reset()`'s with value 0"]
impl crate::ResetValue for super::DCCISW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DCCISW`"]
pub struct DCCISW_W<'a> {
    w: &'a mut W,
}
impl<'a> DCCISW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - D-cache clean and invalidate by set-way"]
    #[inline(always)]
    pub fn dccisw(&mut self) -> DCCISW_W {
        DCCISW_W { w: self }
    }
}

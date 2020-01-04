#[doc = "Writer for register DCISW"]
pub type W = crate::W<u32, super::DCISW>;
#[doc = "Register DCISW `reset()`'s with value 0"]
impl crate::ResetValue for super::DCISW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DCISW`"]
pub struct DCISW_W<'a> {
    w: &'a mut W,
}
impl<'a> DCISW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - D-cache invalidate by set-way"]
    #[inline(always)]
    pub fn dcisw(&mut self) -> DCISW_W {
        DCISW_W { w: self }
    }
}

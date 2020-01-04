#[doc = "Writer for register DCCSW"]
pub type W = crate::W<u32, super::DCCSW>;
#[doc = "Register DCCSW `reset()`'s with value 0"]
impl crate::ResetValue for super::DCCSW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DCCSW`"]
pub struct DCCSW_W<'a> {
    w: &'a mut W,
}
impl<'a> DCCSW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - D-cache clean by set-way"]
    #[inline(always)]
    pub fn dccsw(&mut self) -> DCCSW_W {
        DCCSW_W { w: self }
    }
}

#[doc = "Writer for register DR_CLEAR"]
pub type W = crate::W<u32, super::DR_CLEAR>;
#[doc = "Register DR_CLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::DR_CLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DR_CLEAR`"]
pub struct DR_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DR_CLEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - DR_CLEAR"]
    #[inline(always)]
    pub fn dr_clear(&mut self) -> DR_CLEAR_W {
        DR_CLEAR_W { w: self }
    }
}

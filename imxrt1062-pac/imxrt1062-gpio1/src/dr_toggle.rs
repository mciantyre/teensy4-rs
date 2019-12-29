#[doc = "Writer for register DR_TOGGLE"]
pub type W = crate::W<u32, super::DR_TOGGLE>;
#[doc = "Register DR_TOGGLE `reset()`'s with value 0"]
impl crate::ResetValue for super::DR_TOGGLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DR_TOGGLE`"]
pub struct DR_TOGGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DR_TOGGLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - DR_TOGGLE"]
    #[inline(always)]
    pub fn dr_toggle(&mut self) -> DR_TOGGLE_W {
        DR_TOGGLE_W { w: self }
    }
}

#[doc = "Writer for register DR_SET"]
pub type W = crate::W<u32, super::DR_SET>;
#[doc = "Register DR_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::DR_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DR_SET`"]
pub struct DR_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DR_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - DR_SET"]
    #[inline(always)]
    pub fn dr_set(&mut self) -> DR_SET_W {
        DR_SET_W { w: self }
    }
}

#[doc = "Reader of register COMP2%s"]
pub type R = crate::R<u16, super::COMP2>;
#[doc = "Writer for register COMP2%s"]
pub type W = crate::W<u16, super::COMP2>;
#[doc = "Register COMP2%s `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPARISON_2`"]
pub type COMPARISON_2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMPARISON_2`"]
pub struct COMPARISON_2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARISON_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Comparison Value 2"]
    #[inline(always)]
    pub fn comparison_2(&self) -> COMPARISON_2_R {
        COMPARISON_2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Comparison Value 2"]
    #[inline(always)]
    pub fn comparison_2(&mut self) -> COMPARISON_2_W {
        COMPARISON_2_W { w: self }
    }
}

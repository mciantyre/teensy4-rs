#[doc = "Reader of register COMP1%s"]
pub type R = crate::R<u16, super::COMP1>;
#[doc = "Writer for register COMP1%s"]
pub type W = crate::W<u16, super::COMP1>;
#[doc = "Register COMP1%s `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPARISON_1`"]
pub type COMPARISON_1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMPARISON_1`"]
pub struct COMPARISON_1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARISON_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Comparison Value 1"]
    #[inline(always)]
    pub fn comparison_1(&self) -> COMPARISON_1_R {
        COMPARISON_1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Comparison Value 1"]
    #[inline(always)]
    pub fn comparison_1(&mut self) -> COMPARISON_1_W {
        COMPARISON_1_W { w: self }
    }
}

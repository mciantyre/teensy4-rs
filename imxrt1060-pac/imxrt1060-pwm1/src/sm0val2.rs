#[doc = "Reader of register SM0VAL2"]
pub type R = crate::R<u16, super::SM0VAL2>;
#[doc = "Writer for register SM0VAL2"]
pub type W = crate::W<u16, super::SM0VAL2>;
#[doc = "Register SM0VAL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SM0VAL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VAL2`"]
pub type VAL2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VAL2`"]
pub struct VAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Value Register 2"]
    #[inline(always)]
    pub fn val2(&self) -> VAL2_R {
        VAL2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value Register 2"]
    #[inline(always)]
    pub fn val2(&mut self) -> VAL2_W {
        VAL2_W { w: self }
    }
}

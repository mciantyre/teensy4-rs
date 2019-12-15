#[doc = "Reader of register SM0VAL0"]
pub type R = crate::R<u16, super::SM0VAL0>;
#[doc = "Writer for register SM0VAL0"]
pub type W = crate::W<u16, super::SM0VAL0>;
#[doc = "Register SM0VAL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SM0VAL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VAL0`"]
pub type VAL0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VAL0`"]
pub struct VAL0_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Value Register 0"]
    #[inline(always)]
    pub fn val0(&self) -> VAL0_R {
        VAL0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value Register 0"]
    #[inline(always)]
    pub fn val0(&mut self) -> VAL0_W {
        VAL0_W { w: self }
    }
}

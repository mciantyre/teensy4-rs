#[doc = "Reader of register SMVAL1"]
pub type R = crate::R<u16, super::SMVAL1>;
#[doc = "Writer for register SMVAL1"]
pub type W = crate::W<u16, super::SMVAL1>;
#[doc = "Register SMVAL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SMVAL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VAL1`"]
pub type VAL1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VAL1`"]
pub struct VAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Value Register 1"]
    #[inline(always)]
    pub fn val1(&self) -> VAL1_R {
        VAL1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value Register 1"]
    #[inline(always)]
    pub fn val1(&mut self) -> VAL1_W {
        VAL1_W { w: self }
    }
}

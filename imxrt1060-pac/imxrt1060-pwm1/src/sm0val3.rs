#[doc = "Reader of register SM0VAL3"]
pub type R = crate::R<u16, super::SM0VAL3>;
#[doc = "Writer for register SM0VAL3"]
pub type W = crate::W<u16, super::SM0VAL3>;
#[doc = "Register SM0VAL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SM0VAL3 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VAL3`"]
pub type VAL3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VAL3`"]
pub struct VAL3_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Value Register 3"]
    #[inline(always)]
    pub fn val3(&self) -> VAL3_R {
        VAL3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value Register 3"]
    #[inline(always)]
    pub fn val3(&mut self) -> VAL3_W {
        VAL3_W { w: self }
    }
}

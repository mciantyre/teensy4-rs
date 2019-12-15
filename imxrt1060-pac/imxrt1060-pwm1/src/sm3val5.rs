#[doc = "Reader of register SM3VAL5"]
pub type R = crate::R<u16, super::SM3VAL5>;
#[doc = "Writer for register SM3VAL5"]
pub type W = crate::W<u16, super::SM3VAL5>;
#[doc = "Register SM3VAL5 `reset()`'s with value 0"]
impl crate::ResetValue for super::SM3VAL5 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VAL5`"]
pub type VAL5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VAL5`"]
pub struct VAL5_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Value Register 5"]
    #[inline(always)]
    pub fn val5(&self) -> VAL5_R {
        VAL5_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value Register 5"]
    #[inline(always)]
    pub fn val5(&mut self) -> VAL5_W {
        VAL5_W { w: self }
    }
}

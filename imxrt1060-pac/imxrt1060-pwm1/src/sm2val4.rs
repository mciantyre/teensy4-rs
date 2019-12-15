#[doc = "Reader of register SM2VAL4"]
pub type R = crate::R<u16, super::SM2VAL4>;
#[doc = "Writer for register SM2VAL4"]
pub type W = crate::W<u16, super::SM2VAL4>;
#[doc = "Register SM2VAL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SM2VAL4 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VAL4`"]
pub type VAL4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VAL4`"]
pub struct VAL4_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Value Register 4"]
    #[inline(always)]
    pub fn val4(&self) -> VAL4_R {
        VAL4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value Register 4"]
    #[inline(always)]
    pub fn val4(&mut self) -> VAL4_W {
        VAL4_W { w: self }
    }
}

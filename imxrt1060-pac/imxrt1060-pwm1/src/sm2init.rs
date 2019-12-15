#[doc = "Reader of register SM2INIT"]
pub type R = crate::R<u16, super::SM2INIT>;
#[doc = "Writer for register SM2INIT"]
pub type W = crate::W<u16, super::SM2INIT>;
#[doc = "Register SM2INIT `reset()`'s with value 0"]
impl crate::ResetValue for super::SM2INIT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INIT`"]
pub type INIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INIT`"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Initial Count Register Bits"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Initial Count Register Bits"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
}

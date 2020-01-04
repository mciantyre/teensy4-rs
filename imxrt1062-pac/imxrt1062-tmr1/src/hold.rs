#[doc = "Reader of register HOLD%s"]
pub type R = crate::R<u16, super::HOLD>;
#[doc = "Writer for register HOLD%s"]
pub type W = crate::W<u16, super::HOLD>;
#[doc = "Register HOLD%s `reset()`'s with value 0"]
impl crate::ResetValue for super::HOLD {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOLD`"]
pub type HOLD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HOLD`"]
pub struct HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> HOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This read/write register stores the counter's values of specific channels whenever any of the four counters within a module is read"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This read/write register stores the counter's values of specific channels whenever any of the four counters within a module is read"]
    #[inline(always)]
    pub fn hold(&mut self) -> HOLD_W {
        HOLD_W { w: self }
    }
}

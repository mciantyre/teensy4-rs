#[doc = "Reader of register CNTR%s"]
pub type R = crate::R<u16, super::CNTR>;
#[doc = "Writer for register CNTR%s"]
pub type W = crate::W<u16, super::CNTR>;
#[doc = "Register CNTR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CNTR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNTER`"]
pub type COUNTER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNTER`"]
pub struct COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This read/write register is the counter for the corresponding channel in a timer module."]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This read/write register is the counter for the corresponding channel in a timer module."]
    #[inline(always)]
    pub fn counter(&mut self) -> COUNTER_W {
        COUNTER_W { w: self }
    }
}

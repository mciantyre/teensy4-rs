#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Writer for register ISR"]
pub type W = crate::W<u32, super::ISR>;
#[doc = "Register ISR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISR`"]
pub type ISR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISR`"]
pub struct ISR_W<'a> {
    w: &'a mut W,
}
impl<'a> ISR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISR"]
    #[inline(always)]
    pub fn isr(&self) -> ISR_R {
        ISR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISR"]
    #[inline(always)]
    pub fn isr(&mut self) -> ISR_W {
        ISR_W { w: self }
    }
}

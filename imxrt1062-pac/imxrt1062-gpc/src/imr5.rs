#[doc = "Reader of register IMR5"]
pub type R = crate::R<u32, super::IMR5>;
#[doc = "Writer for register IMR5"]
pub type W = crate::W<u32, super::IMR5>;
#[doc = "Register IMR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::IMR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IMR5`"]
pub type IMR5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IMR5`"]
pub struct IMR5_W<'a> {
    w: &'a mut W,
}
impl<'a> IMR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IRQ\\[159:128\\]
masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    pub fn imr5(&self) -> IMR5_R {
        IMR5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IRQ\\[159:128\\]
masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    pub fn imr5(&mut self) -> IMR5_W {
        IMR5_W { w: self }
    }
}

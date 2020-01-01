#[doc = "Reader of register IMR4"]
pub type R = crate::R<u32, super::IMR4>;
#[doc = "Writer for register IMR4"]
pub type W = crate::W<u32, super::IMR4>;
#[doc = "Register IMR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::IMR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IMR4`"]
pub type IMR4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IMR4`"]
pub struct IMR4_W<'a> {
    w: &'a mut W,
}
impl<'a> IMR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IRQ\\[127:96\\]
masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    pub fn imr4(&self) -> IMR4_R {
        IMR4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IRQ\\[127:96\\]
masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    pub fn imr4(&mut self) -> IMR4_W {
        IMR4_W { w: self }
    }
}

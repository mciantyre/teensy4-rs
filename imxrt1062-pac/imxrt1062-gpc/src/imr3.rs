#[doc = "Reader of register IMR3"]
pub type R = crate::R<u32, super::IMR3>;
#[doc = "Writer for register IMR3"]
pub type W = crate::W<u32, super::IMR3>;
#[doc = "Register IMR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::IMR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IMR3`"]
pub type IMR3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IMR3`"]
pub struct IMR3_W<'a> {
    w: &'a mut W,
}
impl<'a> IMR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IRQ\\[95:64\\]
masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    pub fn imr3(&self) -> IMR3_R {
        IMR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IRQ\\[95:64\\]
masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    pub fn imr3(&mut self) -> IMR3_W {
        IMR3_W { w: self }
    }
}

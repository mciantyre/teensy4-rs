#[doc = "Reader of register CLKPRESCALER"]
pub type R = crate::R<u8, super::CLKPRESCALER>;
#[doc = "Writer for register CLKPRESCALER"]
pub type W = crate::W<u8, super::CLKPRESCALER>;
#[doc = "Register CLKPRESCALER `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKPRESCALER {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLK_DIV`"]
pub type CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLK_DIV`"]
pub struct CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CLK_DIV"]
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CLK_DIV"]
    #[inline(always)]
    pub fn clk_div(&mut self) -> CLK_DIV_W {
        CLK_DIV_W { w: self }
    }
}

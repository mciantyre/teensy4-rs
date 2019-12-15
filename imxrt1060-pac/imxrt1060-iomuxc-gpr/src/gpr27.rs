#[doc = "Reader of register GPR27"]
pub type R = crate::R<u32, super::GPR27>;
#[doc = "Writer for register GPR27"]
pub type W = crate::W<u32, super::GPR27>;
#[doc = "Register GPR27 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR27 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_MUX2_GPIO_SEL`"]
pub type GPIO_MUX2_GPIO_SEL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_MUX2_GPIO_SEL`"]
pub struct GPIO_MUX2_GPIO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_MUX2_GPIO_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO2 and GPIO7 share same IO MUX function, GPIO_MUX2 selects one GPIO function."]
    #[inline(always)]
    pub fn gpio_mux2_gpio_sel(&self) -> GPIO_MUX2_GPIO_SEL_R {
        GPIO_MUX2_GPIO_SEL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO2 and GPIO7 share same IO MUX function, GPIO_MUX2 selects one GPIO function."]
    #[inline(always)]
    pub fn gpio_mux2_gpio_sel(&mut self) -> GPIO_MUX2_GPIO_SEL_W {
        GPIO_MUX2_GPIO_SEL_W { w: self }
    }
}

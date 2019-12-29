#[doc = "Reader of register PLL_SYS_DENOM"]
pub type R = crate::R<u32, super::PLL_SYS_DENOM>;
#[doc = "Writer for register PLL_SYS_DENOM"]
pub type W = crate::W<u32, super::PLL_SYS_DENOM>;
#[doc = "Register PLL_SYS_DENOM `reset()`'s with value 0x12"]
impl crate::ResetValue for super::PLL_SYS_DENOM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x12
    }
}
#[doc = "Reader of field `B`"]
pub type B_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `B`"]
pub struct B_W<'a> {
    w: &'a mut W,
}
impl<'a> B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - 30 bit Denominator (B) of fractional loop divider (unsigned integer)."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - 30 bit Denominator (B) of fractional loop divider (unsigned integer)."]
    #[inline(always)]
    pub fn b(&mut self) -> B_W {
        B_W { w: self }
    }
}

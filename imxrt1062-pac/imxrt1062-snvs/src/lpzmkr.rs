#[doc = "Reader of register LPZMKR[%s]"]
pub type R = crate::R<u32, super::LPZMKR>;
#[doc = "Writer for register LPZMKR[%s]"]
pub type W = crate::W<u32, super::LPZMKR>;
#[doc = "Register LPZMKR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::LPZMKR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ZMK`"]
pub type ZMK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ZMK`"]
pub struct ZMK_W<'a> {
    w: &'a mut W,
}
impl<'a> ZMK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Zeroizable Master Key Each of these registers contains 32 bits of the 256-bit ZMK value"]
    #[inline(always)]
    pub fn zmk(&self) -> ZMK_R {
        ZMK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Zeroizable Master Key Each of these registers contains 32 bits of the 256-bit ZMK value"]
    #[inline(always)]
    pub fn zmk(&mut self) -> ZMK_W {
        ZMK_W { w: self }
    }
}

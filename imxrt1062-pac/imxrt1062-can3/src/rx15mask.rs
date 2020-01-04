#[doc = "Reader of register RX15MASK"]
pub type R = crate::R<u32, super::RX15MASK>;
#[doc = "Writer for register RX15MASK"]
pub type W = crate::W<u32, super::RX15MASK>;
#[doc = "Register RX15MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::RX15MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX15M`"]
pub type RX15M_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RX15M`"]
pub struct RX15M_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m(&self) -> RX15M_R {
        RX15M_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m(&mut self) -> RX15M_W {
        RX15M_W { w: self }
    }
}

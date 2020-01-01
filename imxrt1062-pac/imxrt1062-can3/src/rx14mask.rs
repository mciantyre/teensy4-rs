#[doc = "Reader of register RX14MASK"]
pub type R = crate::R<u32, super::RX14MASK>;
#[doc = "Writer for register RX14MASK"]
pub type W = crate::W<u32, super::RX14MASK>;
#[doc = "Register RX14MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::RX14MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX14M`"]
pub type RX14M_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RX14M`"]
pub struct RX14M_W<'a> {
    w: &'a mut W,
}
impl<'a> RX14M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m(&self) -> RX14M_R {
        RX14M_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m(&mut self) -> RX14M_W {
        RX14M_W { w: self }
    }
}

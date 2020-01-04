#[doc = "Reader of register TIPG"]
pub type R = crate::R<u32, super::TIPG>;
#[doc = "Writer for register TIPG"]
pub type W = crate::W<u32, super::TIPG>;
#[doc = "Register TIPG `reset()`'s with value 0x0c"]
impl crate::ResetValue for super::TIPG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c
    }
}
#[doc = "Reader of field `IPG`"]
pub type IPG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IPG`"]
pub struct IPG_W<'a> {
    w: &'a mut W,
}
impl<'a> IPG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Transmit Inter-Packet Gap"]
    #[inline(always)]
    pub fn ipg(&self) -> IPG_R {
        IPG_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit Inter-Packet Gap"]
    #[inline(always)]
    pub fn ipg(&mut self) -> IPG_W {
        IPG_W { w: self }
    }
}

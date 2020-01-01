#[doc = "Reader of register STL"]
pub type R = crate::R<u32, super::STL>;
#[doc = "Writer for register STL"]
pub type W = crate::W<u32, super::STL>;
#[doc = "Register STL `reset()`'s with value 0"]
impl crate::ResetValue for super::STL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TxDataLeft`"]
pub struct TXDATALEFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATALEFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:23 - SPDIF transmit left channel data. It is write-only, and always returns zeros when read"]
    #[inline(always)]
    pub fn tx_data_left(&mut self) -> TXDATALEFT_W {
        TXDATALEFT_W { w: self }
    }
}

#[doc = "Reader of register STR"]
pub type R = crate::R<u32, super::STR>;
#[doc = "Writer for register STR"]
pub type W = crate::W<u32, super::STR>;
#[doc = "Register STR `reset()`'s with value 0"]
impl crate::ResetValue for super::STR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TxDataRight`"]
pub struct TXDATARIGHT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATARIGHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:23 - SPDIF transmit right channel data. It is write-only, and always returns zeros when read"]
    #[inline(always)]
    pub fn tx_data_right(&mut self) -> TXDATARIGHT_W {
        TXDATARIGHT_W { w: self }
    }
}

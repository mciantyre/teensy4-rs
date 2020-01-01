#[doc = "Reader of register TAEM"]
pub type R = crate::R<u32, super::TAEM>;
#[doc = "Writer for register TAEM"]
pub type W = crate::W<u32, super::TAEM>;
#[doc = "Register TAEM `reset()`'s with value 0x04"]
impl crate::ResetValue for super::TAEM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `TX_ALMOST_EMPTY`"]
pub type TX_ALMOST_EMPTY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_ALMOST_EMPTY`"]
pub struct TX_ALMOST_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ALMOST_EMPTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Value of Transmit FIFO Almost Empty Threshold"]
    #[inline(always)]
    pub fn tx_almost_empty(&self) -> TX_ALMOST_EMPTY_R {
        TX_ALMOST_EMPTY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value of Transmit FIFO Almost Empty Threshold"]
    #[inline(always)]
    pub fn tx_almost_empty(&mut self) -> TX_ALMOST_EMPTY_W {
        TX_ALMOST_EMPTY_W { w: self }
    }
}

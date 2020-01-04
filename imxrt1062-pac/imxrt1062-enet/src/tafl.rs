#[doc = "Reader of register TAFL"]
pub type R = crate::R<u32, super::TAFL>;
#[doc = "Writer for register TAFL"]
pub type W = crate::W<u32, super::TAFL>;
#[doc = "Register TAFL `reset()`'s with value 0x08"]
impl crate::ResetValue for super::TAFL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `TX_ALMOST_FULL`"]
pub type TX_ALMOST_FULL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_ALMOST_FULL`"]
pub struct TX_ALMOST_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ALMOST_FULL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Value Of The Transmit FIFO Almost Full Threshold"]
    #[inline(always)]
    pub fn tx_almost_full(&self) -> TX_ALMOST_FULL_R {
        TX_ALMOST_FULL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value Of The Transmit FIFO Almost Full Threshold"]
    #[inline(always)]
    pub fn tx_almost_full(&mut self) -> TX_ALMOST_FULL_W {
        TX_ALMOST_FULL_W { w: self }
    }
}

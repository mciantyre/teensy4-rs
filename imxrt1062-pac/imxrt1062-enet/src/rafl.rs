#[doc = "Reader of register RAFL"]
pub type R = crate::R<u32, super::RAFL>;
#[doc = "Writer for register RAFL"]
pub type W = crate::W<u32, super::RAFL>;
#[doc = "Register RAFL `reset()`'s with value 0x04"]
impl crate::ResetValue for super::RAFL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `RX_ALMOST_FULL`"]
pub type RX_ALMOST_FULL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_ALMOST_FULL`"]
pub struct RX_ALMOST_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ALMOST_FULL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Value Of The Receive FIFO Almost Full Threshold"]
    #[inline(always)]
    pub fn rx_almost_full(&self) -> RX_ALMOST_FULL_R {
        RX_ALMOST_FULL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value Of The Receive FIFO Almost Full Threshold"]
    #[inline(always)]
    pub fn rx_almost_full(&mut self) -> RX_ALMOST_FULL_W {
        RX_ALMOST_FULL_W { w: self }
    }
}

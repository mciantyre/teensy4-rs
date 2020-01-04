#[doc = "Reader of register STCSCL"]
pub type R = crate::R<u32, super::STCSCL>;
#[doc = "Writer for register STCSCL"]
pub type W = crate::W<u32, super::STCSCL>;
#[doc = "Register STCSCL `reset()`'s with value 0"]
impl crate::ResetValue for super::STCSCL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TxCChannelCons_l`"]
pub type TXCCHANNELCONS_L_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TxCChannelCons_l`"]
pub struct TXCCHANNELCONS_L_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCCHANNELCONS_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - SPDIF transmit Cons"]
    #[inline(always)]
    pub fn tx_cchannel_cons_l(&self) -> TXCCHANNELCONS_L_R {
        TXCCHANNELCONS_L_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - SPDIF transmit Cons"]
    #[inline(always)]
    pub fn tx_cchannel_cons_l(&mut self) -> TXCCHANNELCONS_L_W {
        TXCCHANNELCONS_L_W { w: self }
    }
}

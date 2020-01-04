#[doc = "Reader of register STCSCH"]
pub type R = crate::R<u32, super::STCSCH>;
#[doc = "Writer for register STCSCH"]
pub type W = crate::W<u32, super::STCSCH>;
#[doc = "Register STCSCH `reset()`'s with value 0"]
impl crate::ResetValue for super::STCSCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TxCChannelCons_h`"]
pub type TXCCHANNELCONS_H_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TxCChannelCons_h`"]
pub struct TXCCHANNELCONS_H_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCCHANNELCONS_H_W<'a> {
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
    pub fn tx_cchannel_cons_h(&self) -> TXCCHANNELCONS_H_R {
        TXCCHANNELCONS_H_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - SPDIF transmit Cons"]
    #[inline(always)]
    pub fn tx_cchannel_cons_h(&mut self) -> TXCCHANNELCONS_H_W {
        TXCCHANNELCONS_H_W { w: self }
    }
}

#[doc = "Reader of register SRCSL"]
pub type R = crate::R<u32, super::SRCSL>;
#[doc = "Reader of field `RxCChannel_l`"]
pub type RXCCHANNEL_L_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - SPDIF receive C channel register, contains next 24 bits of C channel without interpretation"]
    #[inline(always)]
    pub fn rx_cchannel_l(&self) -> RXCCHANNEL_L_R {
        RXCCHANNEL_L_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}

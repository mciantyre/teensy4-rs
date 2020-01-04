#[doc = "Reader of register SRCSH"]
pub type R = crate::R<u32, super::SRCSH>;
#[doc = "Reader of field `RxCChannel_h`"]
pub type RXCCHANNEL_H_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - SPDIF receive C channel register, contains first 24 bits of C channel without interpretation"]
    #[inline(always)]
    pub fn rx_cchannel_h(&self) -> RXCCHANNEL_H_R {
        RXCCHANNEL_H_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}

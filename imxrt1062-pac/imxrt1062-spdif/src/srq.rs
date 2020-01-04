#[doc = "Reader of register SRQ"]
pub type R = crate::R<u32, super::SRQ>;
#[doc = "Reader of field `RxQChannel`"]
pub type RXQCHANNEL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - SPDIF receive Q channel register, contains next 3 Q channel bytes"]
    #[inline(always)]
    pub fn rx_qchannel(&self) -> RXQCHANNEL_R {
        RXQCHANNEL_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}

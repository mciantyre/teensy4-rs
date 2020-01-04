#[doc = "Reader of register SRU"]
pub type R = crate::R<u32, super::SRU>;
#[doc = "Reader of field `RxUChannel`"]
pub type RXUCHANNEL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - SPDIF receive U channel register, contains next 3 U channel bytes"]
    #[inline(always)]
    pub fn rx_uchannel(&self) -> RXUCHANNEL_R {
        RXUCHANNEL_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}

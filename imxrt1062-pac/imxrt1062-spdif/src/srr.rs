#[doc = "Reader of register SRR"]
pub type R = crate::R<u32, super::SRR>;
#[doc = "Reader of field `RxDataRight`"]
pub type RXDATARIGHT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Processor receive SPDIF data right"]
    #[inline(always)]
    pub fn rx_data_right(&self) -> RXDATARIGHT_R {
        RXDATARIGHT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}

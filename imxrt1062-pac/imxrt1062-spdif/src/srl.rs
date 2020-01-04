#[doc = "Reader of register SRL"]
pub type R = crate::R<u32, super::SRL>;
#[doc = "Reader of field `RxDataLeft`"]
pub type RXDATALEFT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Processor receive SPDIF data left"]
    #[inline(always)]
    pub fn rx_data_left(&self) -> RXDATALEFT_R {
        RXDATALEFT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}

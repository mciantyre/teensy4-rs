#[doc = "Reader of register MFSR"]
pub type R = crate::R<u32, super::MFSR>;
#[doc = "Reader of field `TXCOUNT`"]
pub type TXCOUNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXCOUNT`"]
pub type RXCOUNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Transmit FIFO Count"]
    #[inline(always)]
    pub fn txcount(&self) -> TXCOUNT_R {
        TXCOUNT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Receive FIFO Count"]
    #[inline(always)]
    pub fn rxcount(&self) -> RXCOUNT_R {
        RXCOUNT_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}

#[doc = "Reader of register HWTXBUF"]
pub type R = crate::R<u32, super::HWTXBUF>;
#[doc = "Reader of field `TXBURST`"]
pub type TXBURST_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXCHANADD`"]
pub type TXCHANADD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Default burst size for memory to TX buffer transfer"]
    #[inline(always)]
    pub fn txburst(&self) -> TXBURST_R {
        TXBURST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TX FIFO Buffer size is: (2^TXCHANADD) * 4 Bytes"]
    #[inline(always)]
    pub fn txchanadd(&self) -> TXCHANADD_R {
        TXCHANADD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}

#[doc = "Reader of register HWRXBUF"]
pub type R = crate::R<u32, super::HWRXBUF>;
#[doc = "Reader of field `RXBURST`"]
pub type RXBURST_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXADD`"]
pub type RXADD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Default burst size for memory to RX buffer transfer"]
    #[inline(always)]
    pub fn rxburst(&self) -> RXBURST_R {
        RXBURST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Buffer total size for all receive endpoints is (2^RXADD)"]
    #[inline(always)]
    pub fn rxadd(&self) -> RXADD_R {
        RXADD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}

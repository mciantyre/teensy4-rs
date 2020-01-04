#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Reader of field `TXFIFO`"]
pub type TXFIFO_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXFIFO`"]
pub type RXFIFO_R = crate::R<u8, u8>;
#[doc = "Reader of field `PCSNUM`"]
pub type PCSNUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO Size"]
    #[inline(always)]
    pub fn txfifo(&self) -> TXFIFO_R {
        TXFIFO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive FIFO Size"]
    #[inline(always)]
    pub fn rxfifo(&self) -> RXFIFO_R {
        RXFIFO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PCS Number"]
    #[inline(always)]
    pub fn pcsnum(&self) -> PCSNUM_R {
        PCSNUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}

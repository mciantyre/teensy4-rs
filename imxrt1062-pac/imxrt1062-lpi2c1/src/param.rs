#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Reader of field `MTXFIFO`"]
pub type MTXFIFO_R = crate::R<u8, u8>;
#[doc = "Reader of field `MRXFIFO`"]
pub type MRXFIFO_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Master Transmit FIFO Size"]
    #[inline(always)]
    pub fn mtxfifo(&self) -> MTXFIFO_R {
        MTXFIFO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Master Receive FIFO Size"]
    #[inline(always)]
    pub fn mrxfifo(&self) -> MRXFIFO_R {
        MRXFIFO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}

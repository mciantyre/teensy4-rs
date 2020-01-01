#[doc = "Reader of register RXFIR"]
pub type R = crate::R<u32, super::RXFIR>;
#[doc = "Reader of field `IDHIT`"]
pub type IDHIT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:8 - This 9-bit field indicates which Identifier Acceptance Filter (see Rx FIFO Structure) was hit by the received message that is in the output of the Rx FIFO"]
    #[inline(always)]
    pub fn idhit(&self) -> IDHIT_R {
        IDHIT_R::new((self.bits & 0x01ff) as u16)
    }
}

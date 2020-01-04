#[doc = "Reader of register IPRXFSTS"]
pub type R = crate::R<u32, super::IPRXFSTS>;
#[doc = "Reader of field `FILL`"]
pub type FILL_R = crate::R<u8, u8>;
#[doc = "Reader of field `RDCNTR`"]
pub type RDCNTR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Fill level of IP RX FIFO."]
    #[inline(always)]
    pub fn fill(&self) -> FILL_R {
        FILL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Total Read Data Counter: RDCNTR * 64 Bits."]
    #[inline(always)]
    pub fn rdcntr(&self) -> RDCNTR_R {
        RDCNTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}

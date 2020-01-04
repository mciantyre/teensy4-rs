#[doc = "Reader of register IPTXFSTS"]
pub type R = crate::R<u32, super::IPTXFSTS>;
#[doc = "Reader of field `FILL`"]
pub type FILL_R = crate::R<u8, u8>;
#[doc = "Reader of field `WRCNTR`"]
pub type WRCNTR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Fill level of IP TX FIFO."]
    #[inline(always)]
    pub fn fill(&self) -> FILL_R {
        FILL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Total Write Data Counter: WRCNTR * 64 Bits."]
    #[inline(always)]
    pub fn wrcntr(&self) -> WRCNTR_R {
        WRCNTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}

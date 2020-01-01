#[doc = "Reader of register FRQCNT"]
pub type R = crate::R<u32, super::FRQCNT>;
#[doc = "Reader of field `FRQ_CT`"]
pub type FRQ_CT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:21 - Frequency Count"]
    #[inline(always)]
    pub fn frq_ct(&self) -> FRQ_CT_R {
        FRQ_CT_R::new((self.bits & 0x003f_ffff) as u32)
    }
}

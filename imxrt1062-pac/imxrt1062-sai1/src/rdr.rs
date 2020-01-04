#[doc = "Reader of register RDR[%s]"]
pub type R = crate::R<u32, super::RDR>;
#[doc = "Reader of field `RDR`"]
pub type RDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Data Register"]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

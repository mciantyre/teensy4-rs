#[doc = "Reader of register IEEE_R_OCTETS_OK"]
pub type R = crate::R<u32, super::IEEE_R_OCTETS_OK>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of octets for frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

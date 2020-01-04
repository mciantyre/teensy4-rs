#[doc = "Reader of register IEEE_T_OCTETS_OK"]
pub type R = crate::R<u32, super::IEEE_T_OCTETS_OK>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Octet count for frames transmitted without error Counts total octets (includes header and FCS fields)."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

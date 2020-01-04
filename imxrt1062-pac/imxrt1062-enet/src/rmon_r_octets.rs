#[doc = "Reader of register RMON_R_OCTETS"]
pub type R = crate::R<u32, super::RMON_R_OCTETS>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of receive octets"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

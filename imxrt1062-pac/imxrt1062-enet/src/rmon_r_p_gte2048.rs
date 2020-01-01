#[doc = "Reader of register RMON_R_P_GTE2048"]
pub type R = crate::R<u32, super::RMON_R_P_GTE2048>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of greater-than-2048-byte recieve packets"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}

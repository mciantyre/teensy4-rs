#[doc = "Reader of register ATSTMP"]
pub type R = crate::R<u32, super::ATSTMP>;
#[doc = "Reader of field `TIMESTAMP`"]
pub type TIMESTAMP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp of the last frame transmitted by the core that had TxBD\\[TS\\]
set"]
    #[inline(always)]
    pub fn timestamp(&self) -> TIMESTAMP_R {
        TIMESTAMP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

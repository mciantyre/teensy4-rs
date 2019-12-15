#[doc = "Reader of register SMCNT"]
pub type R = crate::R<u16, super::SMCNT>;
#[doc = "Reader of field `CNT`"]
pub type CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Register Bits"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}

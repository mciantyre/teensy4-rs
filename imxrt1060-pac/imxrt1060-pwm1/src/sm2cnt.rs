#[doc = "Reader of register SM2CNT"]
pub type R = crate::R<u16, super::SM2CNT>;
#[doc = "Reader of field `CNT`"]
pub type CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Register Bits"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}

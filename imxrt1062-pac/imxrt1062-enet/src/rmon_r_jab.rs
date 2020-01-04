#[doc = "Reader of register RMON_R_JAB"]
pub type R = crate::R<u32, super::RMON_R_JAB>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of receive packets greater than MAX_FL and bad CRC"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}

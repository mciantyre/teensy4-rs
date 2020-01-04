#[doc = "Reader of register RMON_R_P1024TO2047"]
pub type R = crate::R<u32, super::RMON_R_P1024TO2047>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of 1024- to 2047-byte recieve packets"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}

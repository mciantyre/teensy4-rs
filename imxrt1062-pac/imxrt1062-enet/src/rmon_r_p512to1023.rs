#[doc = "Reader of register RMON_R_P512TO1023"]
pub type R = crate::R<u32, super::RMON_R_P512TO1023>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of 512- to 1023-byte recieve packets"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}

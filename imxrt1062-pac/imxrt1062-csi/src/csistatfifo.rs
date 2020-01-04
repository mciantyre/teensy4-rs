#[doc = "Reader of register CSISTATFIFO"]
pub type R = crate::R<u32, super::CSISTATFIFO>;
#[doc = "Reader of field `STAT`"]
pub type STAT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Static data from sensor"]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

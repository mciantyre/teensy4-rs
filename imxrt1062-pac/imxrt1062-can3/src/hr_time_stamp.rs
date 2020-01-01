#[doc = "Reader of register HR_TIME_STAMP[%s]"]
pub type R = crate::R<u32, super::HR_TIME_STAMP>;
#[doc = "Reader of field `TS`"]
pub type TS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - High Resolution Time Stamp"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

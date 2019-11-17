#[doc = "Reader of register ICR1"]
pub type R = crate::R<u32, super::ICR1>;
#[doc = "Reader of field `CAPT`"]
pub type CAPT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Capture Value"]
    #[inline(always)]
    pub fn capt(&self) -> CAPT_R {
        CAPT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

#[doc = "Reader of register CVAL"]
pub type R = crate::R<u32, super::CVAL>;
#[doc = "Reader of field `TVL`"]
pub type TVL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current Timer Value"]
    #[inline(always)]
    pub fn tvl(&self) -> TVL_R {
        TVL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

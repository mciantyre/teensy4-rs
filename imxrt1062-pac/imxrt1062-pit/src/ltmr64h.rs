#[doc = "Reader of register LTMR64H"]
pub type R = crate::R<u32, super::LTMR64H>;
#[doc = "Reader of field `LTH`"]
pub type LTH_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Life Timer value"]
    #[inline(always)]
    pub fn lth(&self) -> LTH_R {
        LTH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

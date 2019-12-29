#[doc = "Reader of register LTMR64L"]
pub type R = crate::R<u32, super::LTMR64L>;
#[doc = "Reader of field `LTL`"]
pub type LTL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Life Timer value"]
    #[inline(always)]
    pub fn ltl(&self) -> LTL_R {
        LTL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

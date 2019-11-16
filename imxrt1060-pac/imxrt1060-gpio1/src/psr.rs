#[doc = "Reader of register PSR"]
pub type R = crate::R<u32, super::PSR>;
#[doc = "Reader of field `PSR`"]
pub type PSR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PSR"]
    #[inline(always)]
    pub fn psr(&self) -> PSR_R {
        PSR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

#[doc = "Reader of register PKRSQ"]
pub type R = crate::R<u32, super::PKRSQ>;
#[doc = "Reader of field `PKR_SQ`"]
pub type PKR_SQ_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Poker Square Calculation Result."]
    #[inline(always)]
    pub fn pkr_sq(&self) -> PKR_SQ_R {
        PKR_SQ_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}

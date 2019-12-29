#[doc = "Reader of register SMCVAL1"]
pub type R = crate::R<u16, super::SMCVAL1>;
#[doc = "Reader of field `CAPTVAL1`"]
pub type CAPTVAL1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAPTVAL1"]
    #[inline(always)]
    pub fn captval1(&self) -> CAPTVAL1_R {
        CAPTVAL1_R::new((self.bits & 0xffff) as u16)
    }
}

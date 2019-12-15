#[doc = "Reader of register SMCVAL3"]
pub type R = crate::R<u16, super::SMCVAL3>;
#[doc = "Reader of field `CAPTVAL3`"]
pub type CAPTVAL3_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAPTVAL3"]
    #[inline(always)]
    pub fn captval3(&self) -> CAPTVAL3_R {
        CAPTVAL3_R::new((self.bits & 0xffff) as u16)
    }
}

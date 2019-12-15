#[doc = "Reader of register SM2CVAL1"]
pub type R = crate::R<u16, super::SM2CVAL1>;
#[doc = "Reader of field `CAPTVAL1`"]
pub type CAPTVAL1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAPTVAL1"]
    #[inline(always)]
    pub fn captval1(&self) -> CAPTVAL1_R {
        CAPTVAL1_R::new((self.bits & 0xffff) as u16)
    }
}

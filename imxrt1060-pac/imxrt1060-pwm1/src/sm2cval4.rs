#[doc = "Reader of register SM2CVAL4"]
pub type R = crate::R<u16, super::SM2CVAL4>;
#[doc = "Reader of field `CAPTVAL4`"]
pub type CAPTVAL4_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAPTVAL4"]
    #[inline(always)]
    pub fn captval4(&self) -> CAPTVAL4_R {
        CAPTVAL4_R::new((self.bits & 0xffff) as u16)
    }
}

#[doc = "Reader of register SM1CVAL2"]
pub type R = crate::R<u16, super::SM1CVAL2>;
#[doc = "Reader of field `CAPTVAL2`"]
pub type CAPTVAL2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAPTVAL2"]
    #[inline(always)]
    pub fn captval2(&self) -> CAPTVAL2_R {
        CAPTVAL2_R::new((self.bits & 0xffff) as u16)
    }
}

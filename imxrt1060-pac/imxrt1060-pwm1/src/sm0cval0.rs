#[doc = "Reader of register SM0CVAL0"]
pub type R = crate::R<u16, super::SM0CVAL0>;
#[doc = "Reader of field `CAPTVAL0`"]
pub type CAPTVAL0_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAPTVAL0"]
    #[inline(always)]
    pub fn captval0(&self) -> CAPTVAL0_R {
        CAPTVAL0_R::new((self.bits & 0xffff) as u16)
    }
}

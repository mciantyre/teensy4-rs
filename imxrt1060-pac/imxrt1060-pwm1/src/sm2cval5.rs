#[doc = "Reader of register SM2CVAL5"]
pub type R = crate::R<u16, super::SM2CVAL5>;
#[doc = "Reader of field `CAPTVAL5`"]
pub type CAPTVAL5_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAPTVAL5"]
    #[inline(always)]
    pub fn captval5(&self) -> CAPTVAL5_R {
        CAPTVAL5_R::new((self.bits & 0xffff) as u16)
    }
}

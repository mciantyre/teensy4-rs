#[doc = "Reader of register SM3CVAL1CYC"]
pub type R = crate::R<u16, super::SM3CVAL1CYC>;
#[doc = "Reader of field `CVAL1CYC`"]
pub type CVAL1CYC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CVAL1CYC"]
    #[inline(always)]
    pub fn cval1cyc(&self) -> CVAL1CYC_R {
        CVAL1CYC_R::new((self.bits & 0x0f) as u8)
    }
}

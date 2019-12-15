#[doc = "Reader of register SM3CVAL4CYC"]
pub type R = crate::R<u16, super::SM3CVAL4CYC>;
#[doc = "Reader of field `CVAL4CYC`"]
pub type CVAL4CYC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CVAL4CYC"]
    #[inline(always)]
    pub fn cval4cyc(&self) -> CVAL4CYC_R {
        CVAL4CYC_R::new((self.bits & 0x0f) as u8)
    }
}

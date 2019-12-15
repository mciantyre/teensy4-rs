#[doc = "Reader of register SM2CVAL2CYC"]
pub type R = crate::R<u16, super::SM2CVAL2CYC>;
#[doc = "Reader of field `CVAL2CYC`"]
pub type CVAL2CYC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CVAL2CYC"]
    #[inline(always)]
    pub fn cval2cyc(&self) -> CVAL2CYC_R {
        CVAL2CYC_R::new((self.bits & 0x0f) as u8)
    }
}

#[doc = "Reader of register SM0CVAL0CYC"]
pub type R = crate::R<u16, super::SM0CVAL0CYC>;
#[doc = "Reader of field `CVAL0CYC`"]
pub type CVAL0CYC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CVAL0CYC"]
    #[inline(always)]
    pub fn cval0cyc(&self) -> CVAL0CYC_R {
        CVAL0CYC_R::new((self.bits & 0x0f) as u8)
    }
}

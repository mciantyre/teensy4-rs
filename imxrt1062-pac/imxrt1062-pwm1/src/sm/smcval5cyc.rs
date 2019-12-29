#[doc = "Reader of register SMCVAL5CYC"]
pub type R = crate::R<u16, super::SMCVAL5CYC>;
#[doc = "Reader of field `CVAL5CYC`"]
pub type CVAL5CYC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CVAL5CYC"]
    #[inline(always)]
    pub fn cval5cyc(&self) -> CVAL5CYC_R {
        CVAL5CYC_R::new((self.bits & 0x0f) as u8)
    }
}

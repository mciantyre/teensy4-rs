#[doc = "Reader of register SCR2C"]
pub type R = crate::R<u32, super::SCR2C>;
#[doc = "Reader of field `R2_0_CT`"]
pub type R2_0_CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `R2_1_CT`"]
pub type R2_1_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - Runs of Zero, Length 2 Count"]
    #[inline(always)]
    pub fn r2_0_ct(&self) -> R2_0_CT_R {
        R2_0_CT_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Runs of One, Length 2 Count"]
    #[inline(always)]
    pub fn r2_1_ct(&self) -> R2_1_CT_R {
        R2_1_CT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}

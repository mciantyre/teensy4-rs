#[doc = "Reader of register SCR4C"]
pub type R = crate::R<u32, super::SCR4C>;
#[doc = "Reader of field `R4_0_CT`"]
pub type R4_0_CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `R4_1_CT`"]
pub type R4_1_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Runs of Zero, Length 4 Count"]
    #[inline(always)]
    pub fn r4_0_ct(&self) -> R4_0_CT_R {
        R4_0_CT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Runs of One, Length 4 Count"]
    #[inline(always)]
    pub fn r4_1_ct(&self) -> R4_1_CT_R {
        R4_1_CT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}

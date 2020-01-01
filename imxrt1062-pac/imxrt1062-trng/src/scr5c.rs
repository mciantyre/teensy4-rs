#[doc = "Reader of register SCR5C"]
pub type R = crate::R<u32, super::SCR5C>;
#[doc = "Reader of field `R5_0_CT`"]
pub type R5_0_CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `R5_1_CT`"]
pub type R5_1_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Runs of Zero, Length 5 Count"]
    #[inline(always)]
    pub fn r5_0_ct(&self) -> R5_0_CT_R {
        R5_0_CT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Runs of One, Length 5 Count"]
    #[inline(always)]
    pub fn r5_1_ct(&self) -> R5_1_CT_R {
        R5_1_CT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}

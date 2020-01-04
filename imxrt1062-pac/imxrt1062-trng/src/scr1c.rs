#[doc = "Reader of register SCR1C"]
pub type R = crate::R<u32, super::SCR1C>;
#[doc = "Reader of field `R1_0_CT`"]
pub type R1_0_CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `R1_1_CT`"]
pub type R1_1_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - Runs of Zero, Length 1 Count"]
    #[inline(always)]
    pub fn r1_0_ct(&self) -> R1_0_CT_R {
        R1_0_CT_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Runs of One, Length 1 Count"]
    #[inline(always)]
    pub fn r1_1_ct(&self) -> R1_1_CT_R {
        R1_1_CT_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}

#[doc = "Reader of register SCR6PC"]
pub type R = crate::R<u32, super::SCR6PC>;
#[doc = "Reader of field `R6P_0_CT`"]
pub type R6P_0_CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `R6P_1_CT`"]
pub type R6P_1_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Runs of Zero, Length 6+ Count"]
    #[inline(always)]
    pub fn r6p_0_ct(&self) -> R6P_0_CT_R {
        R6P_0_CT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Runs of One, Length 6+ Count"]
    #[inline(always)]
    pub fn r6p_1_ct(&self) -> R6P_1_CT_R {
        R6P_1_CT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}

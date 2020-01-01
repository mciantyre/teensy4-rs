#[doc = "Reader of register SCR3C"]
pub type R = crate::R<u32, super::SCR3C>;
#[doc = "Reader of field `R3_0_CT`"]
pub type R3_0_CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `R3_1_CT`"]
pub type R3_1_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - Runs of Zeroes, Length 3 Count"]
    #[inline(always)]
    pub fn r3_0_ct(&self) -> R3_0_CT_R {
        R3_0_CT_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Runs of Ones, Length 3 Count"]
    #[inline(always)]
    pub fn r3_1_ct(&self) -> R3_1_CT_R {
        R3_1_CT_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}

#[doc = "Reader of register PKRCNTBA"]
pub type R = crate::R<u32, super::PKRCNTBA>;
#[doc = "Reader of field `PKR_A_CT`"]
pub type PKR_A_CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `PKR_B_CT`"]
pub type PKR_B_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker Ah Count"]
    #[inline(always)]
    pub fn pkr_a_ct(&self) -> PKR_A_CT_R {
        PKR_A_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker Bh Count"]
    #[inline(always)]
    pub fn pkr_b_ct(&self) -> PKR_B_CT_R {
        PKR_B_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}

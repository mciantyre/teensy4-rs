#[doc = "Reader of register PKRCNTFE"]
pub type R = crate::R<u32, super::PKRCNTFE>;
#[doc = "Reader of field `PKR_E_CT`"]
pub type PKR_E_CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `PKR_F_CT`"]
pub type PKR_F_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker Eh Count"]
    #[inline(always)]
    pub fn pkr_e_ct(&self) -> PKR_E_CT_R {
        PKR_E_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker Fh Count"]
    #[inline(always)]
    pub fn pkr_f_ct(&self) -> PKR_F_CT_R {
        PKR_F_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}

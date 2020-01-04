#[doc = "Reader of register PKRCNTDC"]
pub type R = crate::R<u32, super::PKRCNTDC>;
#[doc = "Reader of field `PKR_C_CT`"]
pub type PKR_C_CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `PKR_D_CT`"]
pub type PKR_D_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker Ch Count"]
    #[inline(always)]
    pub fn pkr_c_ct(&self) -> PKR_C_CT_R {
        PKR_C_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker Dh Count"]
    #[inline(always)]
    pub fn pkr_d_ct(&self) -> PKR_D_CT_R {
        PKR_D_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}

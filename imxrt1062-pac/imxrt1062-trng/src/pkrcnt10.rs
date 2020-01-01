#[doc = "Reader of register PKRCNT10"]
pub type R = crate::R<u32, super::PKRCNT10>;
#[doc = "Reader of field `PKR_0_CT`"]
pub type PKR_0_CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `PKR_1_CT`"]
pub type PKR_1_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker 0h Count"]
    #[inline(always)]
    pub fn pkr_0_ct(&self) -> PKR_0_CT_R {
        PKR_0_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker 1h Count"]
    #[inline(always)]
    pub fn pkr_1_ct(&self) -> PKR_1_CT_R {
        PKR_1_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}

#[doc = "Reader of register PKRCNT32"]
pub type R = crate::R<u32, super::PKRCNT32>;
#[doc = "Reader of field `PKR_2_CT`"]
pub type PKR_2_CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `PKR_3_CT`"]
pub type PKR_3_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker 2h Count"]
    #[inline(always)]
    pub fn pkr_2_ct(&self) -> PKR_2_CT_R {
        PKR_2_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker 3h Count"]
    #[inline(always)]
    pub fn pkr_3_ct(&self) -> PKR_3_CT_R {
        PKR_3_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}

#[doc = "Reader of register PKRCNT76"]
pub type R = crate::R<u32, super::PKRCNT76>;
#[doc = "Reader of field `PKR_6_CT`"]
pub type PKR_6_CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `PKR_7_CT`"]
pub type PKR_7_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker 6h Count"]
    #[inline(always)]
    pub fn pkr_6_ct(&self) -> PKR_6_CT_R {
        PKR_6_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker 7h Count"]
    #[inline(always)]
    pub fn pkr_7_ct(&self) -> PKR_7_CT_R {
        PKR_7_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}

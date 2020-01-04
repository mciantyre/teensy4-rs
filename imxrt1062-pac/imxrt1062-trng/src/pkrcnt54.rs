#[doc = "Reader of register PKRCNT54"]
pub type R = crate::R<u32, super::PKRCNT54>;
#[doc = "Reader of field `PKR_4_CT`"]
pub type PKR_4_CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `PKR_5_CT`"]
pub type PKR_5_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker 4h Count"]
    #[inline(always)]
    pub fn pkr_4_ct(&self) -> PKR_4_CT_R {
        PKR_4_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker 5h Count"]
    #[inline(always)]
    pub fn pkr_5_ct(&self) -> PKR_5_CT_R {
        PKR_5_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}

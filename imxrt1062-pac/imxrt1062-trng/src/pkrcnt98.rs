#[doc = "Reader of register PKRCNT98"]
pub type R = crate::R<u32, super::PKRCNT98>;
#[doc = "Reader of field `PKR_8_CT`"]
pub type PKR_8_CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `PKR_9_CT`"]
pub type PKR_9_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker 8h Count"]
    #[inline(always)]
    pub fn pkr_8_ct(&self) -> PKR_8_CT_R {
        PKR_8_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker 9h Count"]
    #[inline(always)]
    pub fn pkr_9_ct(&self) -> PKR_9_CT_R {
        PKR_9_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}

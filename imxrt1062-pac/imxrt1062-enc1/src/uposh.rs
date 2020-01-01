#[doc = "Reader of register UPOSH"]
pub type R = crate::R<u16, super::UPOSH>;
#[doc = "Reader of field `POSH`"]
pub type POSH_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This read-only register contains a snapshot of the UPOS register."]
    #[inline(always)]
    pub fn posh(&self) -> POSH_R {
        POSH_R::new((self.bits & 0xffff) as u16)
    }
}

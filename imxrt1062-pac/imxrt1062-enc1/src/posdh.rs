#[doc = "Reader of register POSDH"]
pub type R = crate::R<u16, super::POSDH>;
#[doc = "Reader of field `POSDH`"]
pub type POSDH_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This read-only register contains a snapshot of the value of the POSD register"]
    #[inline(always)]
    pub fn posdh(&self) -> POSDH_R {
        POSDH_R::new((self.bits & 0xffff) as u16)
    }
}
